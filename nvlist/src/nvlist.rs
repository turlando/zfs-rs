use binary::Reader;
use crate::internal::Nvstream;
use std::io::Result;

#[derive(Debug)]
pub struct Nvlist {
    nvstream: Nvstream,
}

impl Nvlist {
    pub fn read(r: &mut Reader) -> Result<Self> {
        Ok(Self { nvstream: Nvstream::read(r)? })
    }
}
