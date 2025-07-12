
use super::super::types;
use super::super::bits::*;

#[derive(Debug)]
pub enum Offset {
    Bits32(types::Elf32Off),
    Bits64(types::Elf64Off),
}

impl Offset {
    pub fn build32(offset: &[u8;4], is_little_endian: bool) -> Offset {
        Offset::Bits32(to_u32_from_slice(offset, is_little_endian))
    }

    pub fn build64(offset: &[u8;8], is_little_endian: bool) -> Offset {
        Offset::Bits64(to_u64_from_slice(offset, is_little_endian))
    }

}
