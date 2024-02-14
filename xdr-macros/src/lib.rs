//! XDR: External Data Representation Standard (IETF RFC 4506)

mod r#enum;

use proc_macro::TokenStream;
use syn::{ItemEnum, parse_macro_input};

/// Enumeration
///
/// Enumerations have the same representation as signed integers.
/// Enumerations are handy for describing subsets of the integers.
/// Enumerated data is declared as follows:
///
///     enum { name-identifier = constant, ... } identifier;
///
/// For example, the three colors red, yellow, and blue could be
/// described by an enumerated type:
///
///     enum { RED = 2, YELLOW = 3, BLUE = 5 } colors;
///
/// It is an error to encode as an enum any other integer than those that
/// have been given assignments in the enum declaration.
#[proc_macro_derive(Enum)]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    let e = parse_macro_input!(input as ItemEnum);
    r#enum::derive_enum(e).into()
}
