use std::io::{Result};

use binary::Reader;

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

// TODO: Make private when refactoring is over.
pub const I32_SIZE: usize = 4;

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

// TODO: Make private when refactoring is over.
pub const U32_SIZE: usize = 4;

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

// TODO: Make private when refactoring is over.
pub const I64_SIZE: usize = 8;

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

// TODO: Make private when refactoring is over.
pub const U64_SIZE: usize = 8;

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
