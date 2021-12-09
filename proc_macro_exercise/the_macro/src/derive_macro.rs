use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Index, Member};

pub fn derive_bytes_writable_impl(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();

    let predicates = input
        .generics
        .params
        .iter()
        .filter_map(|param| match param {
            syn::GenericParam::Type(t) => Some(t),
            _ => None,
        })
        .map(|ty| {
            let ident = ty.ident.clone();

            syn::parse_quote!( #ident: BytesWritable )
        });

    let where_clause = if let Some(where_clause) = where_clause {
        let mut where_clause = where_clause.clone();

        for predicate in predicates {
            where_clause.predicates.push(predicate);
        }

        where_clause
    } else {
        syn::parse_quote! {
            where
                #(#predicates),*
        }
    };

    let body_writes = match &input.data {
        syn::Data::Struct(struct_data) => {
            let field_writes =
                struct_data.fields.iter().enumerate().map(|(idx, field)| {
                    let field_name: Member = match &field.ident {
                        Some(ident) => Member::Named(ident.clone()),
                        None => Member::Unnamed(Index {
                            index: idx as u32,
                            span: field.span(),
                        }),
                    };

                    quote! {
                        self.#field_name.write_bytes(&mut write_target)?;
                    }
                });

            field_writes
        }
        _ => panic!("Unsupported derive target"),
    };

    let output = quote! {
        impl #impl_generics BytesWritable for MySpecialStruct #ty_generics #where_clause {
            fn write_bytes<W>(&self, mut write_target: W) -> std::io::Result<()>
            where
                W: std::io::Write {
                #(#body_writes)*

                Ok(())
            }
        }
    };

    output.into()
}
