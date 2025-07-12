use super::super::types;
use super::super::bits::*;

#[derive(Debug)]
pub enum EntryPoint{
    Bits32(types::Elf32Addr),
    Bits64(types::Elf64Addr),
}

impl EntryPoint{
    fn build32(pointer: types::Elf32Addr) -> EntryPoint {
        EntryPoint::Bits32(pointer)
    }
    fn build64(pointer: types::Elf64Addr) -> EntryPoint {
        EntryPoint::Bits64(pointer)
    }

    pub fn build(h: &[u8;4], l: &[u8;4], is32bit: bool, is_little_endian: bool) -> EntryPoint {
        match is32bit {
            true => EntryPoint::build32(to_u32_from_slice(l, is_little_endian)),
            false => EntryPoint::build64(to_u64_from_slices(h, l, is_little_endian)),
        }

    }
}
