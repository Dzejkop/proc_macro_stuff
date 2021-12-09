use quote::quote;
use syn::parse::Parse;
use syn::{parse_macro_input, Block, LitInt, Token};

struct RepeatInput {
    number_of_repeats: LitInt,
    _arrow: Token![=>],
    body: Block,
}

impl Parse for RepeatInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            number_of_repeats: input.parse()?,
            _arrow: input.parse()?,
            body: input.parse()?,
        })
    }
}

pub fn repeat_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as RepeatInput);

    let number_of_repeats: usize =
        input.number_of_repeats.base10_parse().unwrap();
    let repeats = (0..number_of_repeats).map(|_| input.body.clone());

    let output = quote! {
        #(#repeats)*
    };

    output.into()
}
