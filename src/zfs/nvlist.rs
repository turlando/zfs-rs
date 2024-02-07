use std::io::{Error, ErrorKind, Result};
use crate::binary::Reader;

#[derive(Debug)]
pub struct List {
    header: StreamHeader,
    version: Version,
    flags: Vec<Flags>,
}

impl List {
    pub fn read(r: &mut Reader) -> Result<Self> {
        let header = StreamHeader::read(r)?;
        let version = Version::read(r)?;
        let flags = Flags::read(r)?;
        Ok(List { header, version, flags })
    }
}

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

#[derive(Debug)]
enum Version { V0 }

const VERSION_SIZE: usize = 4;

impl Version {
    fn read(r: &mut Reader) -> Result<Self> {
        r.try_read_as::<Self, Error, VERSION_SIZE>(Self::decode)
    }

    fn decode(x: &[u8; VERSION_SIZE]) -> Result<Self> {
        match i32::from_be_bytes(*x) {
            0 => Ok(Version::V0),
            x => Err(Error::new(ErrorKind::InvalidInput, x.to_string()))
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Flags { UniqueName, UniqueNameType }

const FLAGS_SIZE: usize = 4;
const FLAGS_FROM_MASK: [(u32, Flags); 2] = [
    (0x1, Flags::UniqueName),
    (0x2, Flags::UniqueNameType),
];

impl Flags {
    fn read(r: &mut Reader) -> Result<Vec<Self>> {
        r.read_as::<Vec<Self>, FLAGS_SIZE>(Self::decode)
    }

    fn decode(x: &[u8; FLAGS_SIZE]) -> Vec<Self> {
        let i = u32::from_be_bytes(*x);

        // TODO: Err if garbage in i.
        // TODO: Collect into a preallocated Vec with reasonable size.
        FLAGS_FROM_MASK.iter()
            .filter(|(flag, _)| i & *flag != 0)
            .map(|(_, val)| *val)
            .collect()
    }
}
