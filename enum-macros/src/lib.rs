use proc_macro::TokenStream;
use syn::{ItemEnum, TypePath, parse_macro_input};
use enum_macros_common::int_enum;

#[proc_macro_attribute]
pub fn int_enum(attr: TokenStream, input: TokenStream) -> TokenStream {
    let t = parse_macro_input!(attr as TypePath);
    let e = parse_macro_input!(input as ItemEnum);

    int_enum::int_enum(e, t).into()
}
