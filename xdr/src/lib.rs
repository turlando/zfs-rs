//! XDR: External Data Representation Standard (IETF RFC 4506)

mod primitive;
mod string;

pub use xdr_macros::Enum;

pub use crate::primitive::{I32, U32, I64, U64};
pub use crate::string::String;
