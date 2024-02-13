//! XDR: External Data Representation Standard (IETF RFC 4506)

use std::io::{Result};
use crate::binary::Reader;

/// Integer
///
/// An XDR signed integer is a 32-bit datum that encodes an integer in
/// the range \[-2147483648,2147483647\].  The integer is represented in
/// two's complement notation.  The most and least significant bytes are
/// 0 and 3, respectively.  Integers are declared as follows:
///
///     int identifier;
///
///       (MSB)                   (LSB)
///     +-------+-------+-------+-------+
///     |byte 0 |byte 1 |byte 2 |byte 3 |                      INTEGER
///     +-------+-------+-------+-------+
///     <------------32 bits------------>
#[derive(Debug)]
pub struct I32(i32);

const I32_SIZE: usize = 4;

impl I32 {
    pub fn read(r: &mut Reader) -> Result<Self> {
        r.read_as::<Self, I32_SIZE>(Self::decode)
    }

    pub fn decode(x: &[u8; I32_SIZE]) -> Self {
        Self(i32::from_be_bytes(*x))
    }
}

impl From<I32> for i32 {
    fn from(I32(x): I32) -> Self { x }
}

/// Unsigned Integer
///
/// An XDR unsigned integer is a 32-bit datum that encodes a nonnegative
/// integer in the range \[0,4294967295\].  It is represented by an
/// unsigned binary number whose most and least significant bytes are 0
/// and 3, respectively.  An unsigned integer is declared as follows:
///
///     unsigned int identifier;
///
///       (MSB)                   (LSB)
///     +-------+-------+-------+-------+
///     |byte 0 |byte 1 |byte 2 |byte 3 |             UNSIGNED INTEGER
///     +-------+-------+-------+-------+
///     <------------32 bits------------>
#[derive(Debug)]
pub struct U32(u32);

const U32_SIZE: usize = 4;

impl U32 {
    pub fn read(r: &mut Reader) -> Result<Self> {
        r.read_as::<Self, U32_SIZE>(Self::decode)
    }

    pub fn decode(x: &[u8; U32_SIZE]) -> Self {
        Self(u32::from_be_bytes(*x))
    }
}

impl From<U32> for u32 {
    fn from(U32(x): U32) -> Self { x }
}

/// ## Enumeration
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
#[derive(Debug)]
pub struct Enum<'a, T, const N: usize> {
    mapping: &'a EnumMapping<T, N>
}

pub type EnumMapping<T, const N: usize> = [(T, i32); N];

impl<'a, T: Copy, const N: usize> Enum<'a, T, N> {
    pub const fn new(mapping: &'a EnumMapping<T, N>) -> Self { Enum { mapping } }

    pub fn read(&self, r: &mut Reader) -> Result<T> {
        r.read_as::<T, I32_SIZE>(|x| self.decode(x))
    }

    // TODO: During the first implementation this function originally returned
    //       EnumValue<T> where EnumValue is { value: T, int: i32 }. This level
    //       of redirection has since been removed as deemed unnecessary.
    //       Consider adding it back in the future if needed.
    pub fn decode(&self, x: &[u8; I32_SIZE]) -> T {
        let v = I32::decode(x).into();

        // TODO: Return Option<T> or Result<T> instead of just T in order to
        // remove unwrap().
        self.mapping.iter()
            .find(|(_, i)| *i == v)
            .unwrap().0
    }
}

#[derive(Debug)]
pub struct Bitmask<'a, T, const N: usize> {
    mapping: &'a EnumMapping<T, N>
}

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

/// Hyper Integer
///
/// The standard also defines 64-bit (8-byte) numbers called hyper
/// integers.  Their representations are the obvious extensions of
/// integer defined above.  They are represented in two's complement
/// notation.  The most and least significant bytes are 0 and 7,
/// respectively.  Their declarations:
///
///     hyper identifier;
///
///       (MSB)                                                   (LSB)
///     +-------+-------+-------+-------+-------+-------+-------+-------+
///     |byte 0 |byte 1 |byte 2 |byte 3 |byte 4 |byte 5 |byte 6 |byte 7 |
///     +-------+-------+-------+-------+-------+-------+-------+-------+
///     <----------------------------64 bits---------------------------->
///                                                HYPER INTEGER
#[derive(Debug)]
pub struct I64(i64);

const I64_SIZE: usize = 8;

impl I64 {
    pub fn read(r: &mut Reader) -> Result<Self> {
        r.read_as::<Self, I64_SIZE>(Self::decode)
    }

    pub fn decode(x: &[u8; I64_SIZE]) -> Self {
        Self(i64::from_be_bytes(*x))
    }
}

impl From<I64> for i64 {
    fn from(I64(x): I64) -> Self { x }
}

/// ## Unsigned Hyper Integer
///
/// The standard also defines 64-bit (8-byte) numbers called unsigned
/// hyper integers.  Their representations are the obvious extensions of
/// integer defined above.  They are represented in two's complement
/// notation.  The most and least significant bytes are 0 and 7,
/// respectively.  Their declarations:
///
///     unsigned hyper identifier;
///
///       (MSB)                                                   (LSB)
///     +-------+-------+-------+-------+-------+-------+-------+-------+
///     |byte 0 |byte 1 |byte 2 |byte 3 |byte 4 |byte 5 |byte 6 |byte 7 |
///     +-------+-------+-------+-------+-------+-------+-------+-------+
///     <----------------------------64 bits---------------------------->
///                                                UNSIGNED HYPER INTEGER
#[derive(Debug)]
pub struct U64(u64);

const U64_SIZE: usize = 8;

impl U64 {
    pub fn read(r: &mut Reader) -> Result<Self> {
        r.read_as::<Self, U64_SIZE>(Self::decode)
    }

    pub fn decode(x: &[u8; U64_SIZE]) -> Self {
        Self(u64::from_be_bytes(*x))
    }
}

impl From<U64> for u64 {
    fn from(U64(x): U64) -> Self { x }
}
