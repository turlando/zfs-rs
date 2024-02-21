use std::io::Result;

use binary::Reader;

use crate::internal::Nvstream;

#[derive(Debug)]
pub struct Nvlist {
    nvstream: Nvstream,
}

impl Nvlist {
    pub fn read(r: &mut Reader) -> Result<Self> {
        Ok(Self { nvstream: Nvstream::read(r)? })
    }
}
