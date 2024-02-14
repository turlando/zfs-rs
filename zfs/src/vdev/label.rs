use binary::Reader;
use std::io::SeekFrom;
use crate::nvlist::List;
use crate::vdev::physical::Physical;

const LABEL_SIZE: u64 = 256 * 1024;
const LABEL_NVLIST_OFFSET: u64 = 16 * 1024;

#[derive(Debug)]
pub enum LabelNumber { L0, L1, L2, L3, }

fn offset(physical_vdev_size: u64, label: LabelNumber) -> u64 {
    let base_offset = match label {
        LabelNumber::L0 | LabelNumber::L1 => 0,
        LabelNumber::L2 | LabelNumber::L3 => physical_vdev_size - LABEL_SIZE * 2,
    };
    let offset = match label {
        LabelNumber::L0 | LabelNumber::L2 => 0,
        LabelNumber::L1 | LabelNumber::L3 => 1,
    };
    base_offset + offset * LABEL_SIZE
}

pub fn read_nvlist(
    vdev: &Physical,
    number: LabelNumber
) -> std::io::Result<List> {
    let mut r = Reader::new(vdev.file());
    r.seek(SeekFrom::Start(offset(vdev.size()?, number) + LABEL_NVLIST_OFFSET))?;
    List::read(&mut r)
}
