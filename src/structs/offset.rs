
use super::super::types;
use super::super::bits::*;

#[derive(Debug, Clone, Copy)]
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

    pub fn to_u32(&self) -> Result<u32, &'static str> {
        match self {
            Offset::Bits32(value) => Ok(value.clone()),
            _ => Err("Can't cast non bits32 offset"),
        }
    }

    pub fn to_u64(&self) -> Result<u64, &'static str> {
        match self {
            Offset::Bits32(value) => Ok(*value as u64),
            Offset::Bits64(value) => Ok(*value),
        }
    }

}

use std::fmt;

impl fmt::Display for Offset{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
           Offset::Bits32(value) => write!(f, "0x{:04x}", value),
           Offset::Bits64(value) => write!(f, "0x{:08x}", value),
        }
    }
}
