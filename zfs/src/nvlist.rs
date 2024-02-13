use std::io::{Error, ErrorKind, Result};
use enum_macros::int_enum;
use crate::binary::Reader;
use crate::xdr::{Bitmask, Enum, EnumMapping};

#[derive(Debug)]
pub struct List {
    header: StreamHeader,
    version: Version,
    flags: Vec<Flags>,
}

impl List {
    pub fn read(r: &mut Reader) -> Result<Self> {
        let header = StreamHeader::read(r)?;
        let version = Version.read(r)?;
        let flags = Flags.read(r)?;
        Ok(List { header, version, flags })
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

#[derive(Clone, Copy, Debug)]
enum Version { V0 }

const VERSIONS: usize = 1;
const VERSION_MAPPING: EnumMapping<Version, VERSIONS> = [(Version::V0, 0)];

#[allow(non_upper_case_globals)]
const Version: Enum<Version, VERSIONS> = Enum::new(&VERSION_MAPPING);

#[derive(Clone, Copy, Debug)]
enum Flags { UniqueName, UniqueNameType }

const FLAGS: usize = 2;
const FLAGS_MAPPING: EnumMapping<Flags, FLAGS> = [
    (Flags::UniqueName, 0x1),
    (Flags::UniqueNameType, 0x2),
];

#[allow(non_upper_case_globals)]
const Flags: Bitmask<Flags, FLAGS> = Bitmask::new(&FLAGS_MAPPING);
