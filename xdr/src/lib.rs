//! XDR: External Data Representation Standard (IETF RFC 4506)

mod primitive;

pub use crate::primitive::{I32, U32, I64, U64};
pub use xdr_macros::Enum;

use binary::Reader;
use crate::primitive::{I32_SIZE};
use std::io::{Result};

// TODO: Remove this stuff and make a Bitmask macro similar to Enum.
#[derive(Debug)]
pub struct Bitmask<'a, T, const N: usize> {
    mapping: &'a EnumMapping<T, N>
}

pub type EnumMapping<T, const N: usize> = [(T, i32); N];

impl<'a, T: Copy, const N: usize> Bitmask<'a, T, N> {
    pub const fn new(mapping: &'a EnumMapping<T, N>) -> Self {
        Bitmask { mapping }
    }

    pub fn read(&self, r: &mut Reader) -> Result<Vec<T>> {
        r.read_as::<Vec<T>, I32_SIZE>(|x| self.decode(x))
    }

    pub fn decode(&self, x: &[u8; I32_SIZE]) -> Vec<T> {
        let v: i32 = I32::decode(x).into();

        // TODO: Err if garbage in i.
        // TODO: Collect into a preallocated Vec with reasonable size.
        self.mapping.iter()
            .filter(|(_, i)| *i & v != 0)
            .map(|(value, _)| *value)
            .collect()
    }
}
