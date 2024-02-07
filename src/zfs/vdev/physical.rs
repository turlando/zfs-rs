use std::fs::File;
use std::path::Path;
use crate::unix::{BlockDevice, FileType};

#[derive(Debug)]
pub enum Physical {
    File(File),
    Device(BlockDevice),
}

impl Physical {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path.as_ref())?;
        match FileType::from(file.metadata()?.file_type()) {
            FileType::File => Ok(Physical::File(file)),
            FileType::BlockDevice => Ok(Physical::Device(BlockDevice::new(file))),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
        }
    }

    pub fn size(&self) -> std::io::Result<u64> {
        match self {
            Physical::File(file) => Ok(file.metadata()?.len()),
            Physical::Device(device) => Ok(device.size()?)
        }
    }

    pub fn file(&self) -> &File {
        match self {
            Physical::File(file) => file,
            Physical::Device(dev) => dev.file()
        }
    }
}
