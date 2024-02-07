use std::io::BufReader;
use std::io::{Error, ErrorKind, Result};
use std::io::{Read, Seek};

const HEADER_ENCODING_SIZE: usize = 1;
const HEADER_ENDIANNESS_SIZE: usize = 1;

#[derive(Debug)]
pub struct List {
    header: StreamHeader,
}

impl List {
    pub fn read<R: Read + Seek>(r: &mut BufReader<R>) -> Result<Self> {
        let header = StreamHeader::read(r)?;
        Ok(List { header })
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
        Ok(StreamHeader { encoding, endianness })
    }
}

#[derive(Debug)]
enum Encoding { Native, XDR }

impl Encoding {
    fn read<R: Read + Seek>(r: &mut BufReader<R>) -> Result<Self> {
        let mut buf: [u8; HEADER_ENCODING_SIZE] = [0; HEADER_ENCODING_SIZE];
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

impl Endianness {
    fn read<R: Read + Seek>(r: &mut BufReader<R>) -> Result<Self> {
        let mut buf: [u8; HEADER_ENDIANNESS_SIZE] = [0; HEADER_ENDIANNESS_SIZE];
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
