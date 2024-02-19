use binary::Reader;
use std::io::{Error, Result};

/// String
///
/// The standard defines a string of n (numbered 0 through n-1) ASCII
/// bytes to be the number n encoded as an unsigned integer (as described
/// above), and followed by the n bytes of the string.  Byte m of the
/// string always precedes byte m+1 of the string, and byte 0 of the
/// string always follows the string's length.  If n is not a multiple of
/// four, then the n bytes are followed by enough (0 to 3) residual zero
/// bytes, r, to make the total byte count a multiple of four.  Counted
/// byte strings are declared as follows:
///
///     string object<m>;
/// or
///     string object<>;
///
/// The constant m denotes an upper bound of the number of bytes that a
/// string may contain.  If m is not specified, as in the second
/// declaration, it is assumed to be (2**32) - 1, the maximum length.
/// The constant m would normally be found in a protocol specification.
/// For example, a filing protocol may state that a file name can be no
/// longer than 255 bytes, as follows:
///
///     string filename<255>;
///
///        0     1     2     3     4     5   ...
///     +-----+-----+-----+-----+-----+-----+...+-----+-----+...+-----+
///     |        length n       |byte0|byte1|...| n-1 |  0  |...|  0  |
///     +-----+-----+-----+-----+-----+-----+...+-----+-----+...+-----+
///     |<-------4 bytes------->|<------n bytes------>|<---r bytes--->|
///                             |<----n+r (where (n+r) mod 4 = 0)---->|
///                                                              STRING
///
/// It is an error to encode a length greater than the maximum described
/// in the specification.
#[derive(Debug)]
pub struct String(std::string::String);

const STRING_LEN_SIZE: usize = 4;

impl String {
    pub fn read(r: &mut Reader) -> Result<Self> {
        let len = r.read_as::<u32, STRING_LEN_SIZE>(Self::decode_len)?;
        let s = String(r.try_read_to::<std::string::String, Error>(
            len as usize, // FIXME: unsafe conversion from u32 to usize
            Self::decode_str
        )?);
        r.align(4)?;
        Ok(s)
    }

    fn decode_len(x: &[u8; STRING_LEN_SIZE]) -> u32 {
        u32::from_be_bytes(*x)
    }

    fn decode_str(x: Vec<u8>) -> Result<std::string::String> {
        match std::string::String::from_utf8(x) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::other(e)) // TODO: maybe use InvalidInput?
        }
    }
}

impl From<String> for std::string::String {
    fn from(x: String) -> std::string::String { x.0 }
}
