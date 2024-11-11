use proc_macro::TokenStream;
use syn::*;
use quote::quote;

mod record_input;
use record_input::*;


#[proc_macro]
pub fn define_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RecordInput);
    let out = quote! { #input };
    out.into()
}