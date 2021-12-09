use quote::quote;
use syn::parse::Parse;
use syn::{parse_macro_input, Ident, ItemFn, Token};

struct Args {
    path: syn::Path,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ident: Ident = input.parse()?;
        let _eq_token: Token![=] = input.parse()?;

        Ok(Self {
            path: input.parse()?,
        })
    }
}

pub fn log_args_impl(
    attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(attrs as Args);

    let arg_prints: Vec<_> = input
        .sig
        .inputs
        .iter()
        .enumerate()
        .map(|(idx, fn_arg)| {
            let (arg_name, arg_type) = match fn_arg {
                syn::FnArg::Receiver(_) => panic!(),
                syn::FnArg::Typed(pat_type) => {
                    let name = match pat_type.pat.as_ref() {
                        syn::Pat::Ident(ident) => ident.ident.clone(),
                        _ => panic!(),
                    };

                    let ty = pat_type.ty.clone();

                    (name, ty)
                }
            };

            let arg_name_string = arg_name.to_string();
            let arg_type_string = format!("{}", quote!(#arg_type));
            let logger = &args.path;

            quote! {
                #logger!("Argument {} '{}', type '{}', value is {:?}", #idx, #arg_name_string, #arg_type_string, #arg_name)
            }
        })
        .collect();

    let old_block = input.block.clone();
    input.block = syn::parse_quote! {
        {
            #(#arg_prints;)*
            #old_block
        }
    };

    let output = quote!( #input );

    println!("{}", output);

    output.into()
}
