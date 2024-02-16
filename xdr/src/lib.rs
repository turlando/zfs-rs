//! XDR: External Data Representation Standard (IETF RFC 4506)

mod primitive;

pub use crate::primitive::{I32, U32, I64, U64};
pub use xdr_macros::Enum;
