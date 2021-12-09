use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, ItemFn, LitInt, Pat, Path, Stmt, Token};

#[proc_macro]
pub fn identity(input: TokenStream) -> TokenStream {
    println!("{}", input);
    input
}

struct RepeatArgs {
    num: LitInt,
    _arrow: Token![=>],
    body: Stmt,
}

impl Parse for RepeatArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            num: input.parse()?,
            _arrow: input.parse()?,
            body: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn repeat(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as RepeatArgs);

    let stmt = args.body;
    let it = (0..args.num.base10_parse().unwrap()).map(|_| stmt.clone());

    let output = quote! {
        #(#it)*
    };

    output.into()
}

#[proc_macro_attribute]
pub fn log_args(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_fn = parse_macro_input!(input as ItemFn);

    let println_lines: Vec<Stmt> = item_fn
        .sig
        .inputs
        .iter()
        .map(|fn_arg| {
            let fn_arg: Pat = match fn_arg {
                syn::FnArg::Receiver(rec) => syn::parse_quote!(#rec),
                syn::FnArg::Typed(typed) => {
                    let ident = match typed.pat.as_ref() {
                        syn::Pat::Ident(ident) => ident,
                        _ => panic!(),
                    };

                    syn::parse_quote!(#ident)
                }
            };

            let fn_arg_string = quote!(#fn_arg).to_string();

            syn::parse_quote! {
                println!("{} = {:?}", #fn_arg_string, #fn_arg);
            }
        })
        .collect();

    let original_block = item_fn.block.clone();
    item_fn.block = syn::parse_quote! (
        {
            #(#println_lines)*
            #original_block
        }
    );

    let output = quote! {
        #item_fn
    };

    println!("{}", output);

    output.into()
}

#[proc_macro_derive(BytesWritable)]
pub fn derive_bytes_writable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let struct_data = match input.data {
        syn::Data::Struct(struct_data) => struct_data,
        _ => panic!(),
    };

    let write_lines = struct_data.fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();

        let stmt: Stmt = syn::parse_quote! {
            self.#ident.write_bytes(&mut write_target)?;
        };

        stmt
    });

    let output = quote! {
        impl BytesWritable for #struct_name {
            fn write_bytes<W>(&self, mut write_target: W) -> std::io::Result<()>
            where
                W: std::io::Write,
            {
                #(#write_lines)*

                Ok(())
            }
        }
    };

    println!("{}", output);

    output.into()
}
