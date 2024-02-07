use std::io::BufReader;
use std::io::{Error, ErrorKind, Result};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct List {
    header: StreamHeader,
    version: Version,
    flags: Vec<Flags>,
}

impl List {
    pub fn read<R: Read + Seek>(r: &mut BufReader<R>) -> Result<Self> {
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
    fn read<R: Read + Seek>(r: &mut BufReader<R>) -> Result<Self> {
        let encoding = Encoding::read(r)?;
        let endianness = Endianness::read(r)?;
        r.seek(SeekFrom::Current(2))?; // unused reserved bytes
        Ok(StreamHeader { encoding, endianness })
    }
}

#[derive(Debug)]
enum Encoding { Native, XDR }

const ENCODING_SIZE: usize = 1;

impl Encoding {
    fn read<R: Read + Seek>(r: &mut BufReader<R>) -> Result<Self> {
        let mut buf: [u8; ENCODING_SIZE] = [0; ENCODING_SIZE];
        r.read_exact(&mut buf)?;
        Self::decode(buf[0])
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
    fn read<R: Read + Seek>(r: &mut BufReader<R>) -> Result<Self> {
        let mut buf: [u8; ENDIANNESS_SIZE] = [0; ENDIANNESS_SIZE];
        r.read_exact(&mut buf)?;
        Self::decode(buf[0])
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
    fn read<R: Read + Seek>(r: &mut BufReader<R>) -> Result<Self> {
        let mut buf: [u8; VERSION_SIZE] = [0; VERSION_SIZE];
        r.read_exact(&mut buf)?;
        Self::decode(&buf)
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

impl Flags {
    fn read<R: Read + Seek>(r: &mut BufReader<R>) -> Result<Vec<Self>> {
        let mut buf: [u8; FLAGS_SIZE] = [0; FLAGS_SIZE];
        r.read_exact(&mut buf)?;
        Ok(Self::decode(&buf))
    }

    fn decode(x: &[u8; FLAGS_SIZE]) -> Vec<Self> {
        let i = u32::from_be_bytes(*x);

        let map = [
            (0x1, Flags::UniqueName),
            (0x2, Flags::UniqueNameType),
        ];
        
        // TODO: Err if garbage in i.
        // TODO: Collect into a preallocated Vec with reasonable size.
        map.iter().filter(|(flag, _)| i & *flag != 0).map(|(_, val)| *val).collect()
    }
}
