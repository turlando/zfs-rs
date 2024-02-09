use std::io::{Error, ErrorKind, Result};
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
enum Encoding { Native, XDR }

const ENCODING_SIZE: usize = 1;

impl Encoding {
    fn read(r: &mut Reader) -> Result<Self> {
        r.try_read_as::<Self, Error, ENCODING_SIZE>(|x| Self::decode(x[0]))
    }

    fn decode(x: u8) -> Result<Self> {
        match x {
            0 => Ok(Encoding::Native),
            1 => Ok(Encoding::XDR),
            x => Err(Error::new(ErrorKind::InvalidInput, x.to_string()))
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Endianness { Big, Little }

const ENDIANNESS_SIZE: usize = 1;

impl Endianness {
    fn read(r: &mut Reader) -> Result<Self> {
        r.try_read_as::<Self, Error, ENDIANNESS_SIZE>(|x| Self::decode(x[0]))
    }

    fn decode(x: u8) -> Result<Self> {
        match x {
            0 => Ok(Endianness::Big),
            1 => Ok(Endianness::Little),
            x => Err(Error::new(ErrorKind::InvalidInput, x.to_string()))
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
