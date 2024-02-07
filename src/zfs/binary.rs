use std::fs::File;
use std::io::{BufReader, Error, Read, Result, Seek};

// TODO: Reader should be parametric around the Read + Seek trait.
//       However, this would make the code way more complex and verbose,
//       as it would require clients to specify such trait all over the place.
//       Right now wr're using it for reading Files only, and such additional
//       complexity can't be justified (yet).
#[derive(Debug)]
pub struct Reader(BufReader<File>);

impl Reader {
    pub fn new(file: File) -> Self {
        Reader(BufReader::new(file))
    }

    pub fn skip(&mut self, offset: u64) -> Result<u64> {
        // TODO: Remove .try_into().unwrap(). Maybe just accept an i64?
        self.0.seek_relative(offset.try_into().unwrap())?;
        self.0.stream_position()
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        self.0.read_exact(buf)
    }

    pub fn read_as<T, const N: usize>(
        &mut self,
        f: impl FnOnce(&[u8; N]) -> T
    ) -> Result<T> {
        let mut buf = [0u8; N];
        self.read(&mut buf)?;
        Ok(f(&buf))
    }

    pub fn try_read_as<T, E, const N: usize>(
        &mut self,
        f: impl FnOnce(&[u8; N]) -> std::result::Result<T, E>
    ) -> Result<T>
    where
        Error: From<E>
    {
        let mut buf = [0u8; N];
        self.read(&mut buf)?;
        Ok(f(&buf)?)
    }
}
