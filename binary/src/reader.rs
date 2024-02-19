use std::fs::File;
use std::io::{BufReader, Error, Read, Result, Seek, SeekFrom};

// TODO: Reader should be parametric around the Read + Seek trait.
//       However, this would make the code way more complex and verbose,
//       as it would require clients to specify such trait all over the place.
//       Right now wr're using it for reading Files only, and such additional
//       complexity can't be justified (yet).
// TODO: Is it ok to capture a reference to File? Should Reader own it? Who knows.
#[derive(Debug)]
pub struct Reader<'a>(BufReader<&'a File>);

impl<'a> Reader<'a> {
    pub fn new(file: &'a File) -> Self {
        Reader(BufReader::new(file))
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        self.0.read_exact(buf)
    }

    pub fn seek(&mut self, pos: SeekFrom) -> Result<u64>{
        self.0.seek(pos)
    }

    pub fn skip(&mut self, offset: u32) -> Result<u64> {
        self.0.seek_relative(offset.into())?;
        self.0.stream_position()
    }

    pub fn align(&mut self, to: u64) -> Result<u64> {
        let pos = self.0.stream_position()?;
        let dest = (pos + (to - 1)) & !(to - 1);
        self.skip((dest - pos) as u32)
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

    pub fn read_to<T>(
        &mut self,
        len: usize,
        f: impl FnOnce(Vec<u8>) -> T
    ) -> Result<T> {
        let mut buf = vec![0; len];
        self.read(&mut buf)?;
        Ok(f(buf))
    }

    pub fn try_read_to<T, E>(
        &mut self,
        len: usize,
        f: impl FnOnce(Vec<u8>) -> std::result::Result<T, E>
    ) -> Result<T>
    where
        Error: From<E>
    {
        let mut buf = vec![0; len];
        self.read(&mut buf)?;
        Ok(f(buf)?)
    }
}
