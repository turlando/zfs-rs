mod int_enum;

use proc_macro::TokenStream;
use syn::{Ident, ItemEnum, parse_macro_input};

#[proc_macro_attribute]
pub fn int_enum(attr: TokenStream, input: TokenStream) -> TokenStream {
    let type_ = parse_macro_input!(attr as Ident);
    let enum_ = parse_macro_input!(input as ItemEnum);

    int_enum::int_enum(enum_, type_).into()
}
