mod attribute_macro;
mod derive_macro;
mod function_like_macro;

#[proc_macro]
pub fn repeat(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    function_like_macro::repeat_impl(input)
}

#[proc_macro_attribute]
pub fn log_args(
    attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    attribute_macro::log_args_impl(attrs, input)
}

#[proc_macro_derive(BytesWritable)]
pub fn derive_bytes_writable(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    derive_macro::derive_bytes_writable_impl(input)
}
