use binary::Reader;
use enum_macros::int_enum;
use std::io::{Error, ErrorKind, Result};
use xdr::Enum;

#[derive(Debug)]
pub struct Nvstream {
    header: StreamHeader,
    nvlist: Nvlist,
}

impl Nvstream {
    pub fn read(r: &mut Reader) -> Result<Self> {
        let header = StreamHeader::read(r)?;
        let nvlist = match header.encoding {
            Encoding::Native => unimplemented!("Can't read native nvlist"),
            Encoding::XDR => Nvlist::read(r)?,
        };
        Ok(Nvstream { header, nvlist })
    }
}

// Why are Encoding and Endianness not using xdr::Enum? Because they're not
// XDR-encoded enums: they're 1 byte wide values, while enums in XDR are 32 bit
// integers. So for now we're keeping this code instead of using the generic
// implementation. Let's see if it's worth changing this in the furure.
#[derive(Debug)]
struct StreamHeader {
    encoding: Encoding,
    endianness: Endianness,
}

impl StreamHeader {
    fn read(r: &mut Reader) -> Result<Self> {
        let encoding = Encoding::read(r)?;
        let endianness = Endianness::read(r)?;
        r.skip(2)?; // unused reserved bytes
        Ok(StreamHeader { encoding, endianness })
    }
}

#[derive(Debug)]
#[int_enum(u8)]
enum Encoding {
    Native = 0,
    XDR    = 1,
}

const ENCODING_SIZE: usize = 1;

impl Encoding {
    fn read(r: &mut Reader) -> Result<Self> {
        r.try_read_as::<Self, Error, ENCODING_SIZE>(|x| Self::decode(x[0]))
    }

    fn decode(x: u8) -> Result<Self> {
        match Self::try_from(x) {
            Ok(v) => Ok(v),
            Err(n) => Err(Error::new(ErrorKind::InvalidInput, n.to_string()))
        }
    }
}

#[derive(Debug)]
#[int_enum(u8)]
enum Endianness {
    Big    = 0,
    Little = 1,
}

const ENDIANNESS_SIZE: usize = 1;

impl Endianness {
    fn read(r: &mut Reader) -> Result<Self> {
        r.try_read_as::<Self, Error, ENDIANNESS_SIZE>(|x| Self::decode(x[0]))
    }

    fn decode(x: u8) -> Result<Self> {
        match Self::try_from(x) {
            Ok(v) => Ok(v),
            Err(n) => Err(Error::new(ErrorKind::InvalidInput, n.to_string()))
        }
    }
}

#[derive(Debug)]
pub struct Nvlist {
    version: Version,
    flags: Flags,
}

impl Nvlist {
    pub fn read(r: &mut Reader) -> Result<Self> {
        let version = Version::read(r)?;
        let flags = Flags::read(r)?;
        Ok(Nvlist { version, flags })
    }
}

#[derive(Debug, Enum)]
enum Version { V0 = 0 }

#[derive(Debug, Enum)]
enum Flags {
    /// Existing nvpairs with matching names are removed before the new nvpair
    /// is added.
    UniqueName = 0x1,
    /// Existing nvpairs with matching names and data types are removed before
    /// the new nvpair is added.
    UniqueNameType = 0x2,
}
