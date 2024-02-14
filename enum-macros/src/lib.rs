use proc_macro::TokenStream;
use syn::{Ident, ItemEnum, parse_macro_input};
use enum_macros_common::int_enum;

#[proc_macro_attribute]
pub fn int_enum(attr: TokenStream, input: TokenStream) -> TokenStream {
    let r#type = parse_macro_input!(attr as Ident);
    let r#enum = parse_macro_input!(input as ItemEnum);

    int_enum::int_enum(r#enum, r#type).into()
}
