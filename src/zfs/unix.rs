use std::fs::File;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DeviceNumber {
    major: u32,
    minor: u32,
}

impl DeviceNumber {
    pub fn new(n: u64) -> Self {
        Self {
            major: ((n >> 8) & 0xff) as u32,
            minor: (n & 0xff) as u32,
        }
    }

    pub fn from_file(file: &File) -> std::io::Result<Self> {
        Ok(Self::new(file.metadata()?.rdev()))
    }
}

#[derive(Debug)]
pub struct BlockDevice(File);

impl BlockDevice {
    pub fn new(file: File) -> Self {
        BlockDevice(file)
    }

    pub fn size(&self) -> std::io::Result<u64> {
        let path = self.path()?;
        let blocks = std::fs::read_to_string(path)?.trim().parse::<u64>().unwrap();
        Ok(blocks * 512)
    }

    pub fn path(&self) -> std::io::Result<PathBuf> {
        let n = DeviceNumber::from_file(&self.0)?;
        Ok(PathBuf::from(format!("/sys/dev/block/{}:{}/size", n.major, n.minor)))
    }

    pub fn file(&self) -> &File {
        &self.0
    }
}

pub enum FileType {
    Dir,
    File,
    Symlink,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
}

impl From<std::fs::FileType> for FileType {
    fn from(t: std::fs::FileType) -> Self {
        if t.is_dir() {
            return FileType::Dir
        }
        if t.is_file() {
            return FileType::File
        }
        if t.is_symlink() {
            return FileType::Symlink
        }
        if t.is_block_device() {
            return FileType::BlockDevice
        }
        if t.is_char_device() {
            return FileType::CharDevice
        }
        if t.is_fifo() {
            return FileType::Fifo
        }
        if t.is_socket() {
            return FileType::Socket
        }
        unreachable!();
    }
}
