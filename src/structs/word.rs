
use crate::types;
use crate::bits::*;

#[derive(Debug, Clone, Copy)]
pub enum Word {
    Bits32(types::Elf32Off),
    Bits64(types::Elf64Off),
}

impl Word {
    pub fn build32(offset: &[u8;4], is_little_endian: bool) -> Word {
        Word::Bits32(to_u32_from_slice(offset, is_little_endian))
    }

    pub fn build64(offset: &[u8;8], is_little_endian: bool) -> Word {
        Word::Bits64(to_u64_from_slice(offset, is_little_endian))
    }

    pub fn build(h: &[u8;4], l: &[u8;4], is32bit: bool, is_little_endian: bool) -> Word {
        match is32bit {
            true => Word::build32(l, is_little_endian),
            false => {
                let mut combined= [0;8];
                combined[0..4].copy_from_slice(h);
                combined[4..8].copy_from_slice(l);
                Word::build64(&combined, is_little_endian)
            }
        }
    }

    #[allow(dead_code)]
    pub fn to_u32(&self) -> Result<u32, &'static str> {
        match self {
            Word::Bits32(value) => Ok(value.clone()),
            _ => Err("Can't cast non bits32 offset"),
        }
    }

    pub fn to_u64(&self) -> Result<u64, &'static str> {
        match self {
            Word::Bits32(value) => Ok(*value as u64),
            Word::Bits64(value) => Ok(*value),
        }
    }

}

use std::fmt;

impl fmt::LowerHex for Word{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let bit32_width = f.width().unwrap_or(4);
        let bit64_width = f.width().unwrap_or(8);

        match self {
           Word::Bits32(value) => write!(f, "0x{:0bit32_width$x}", value),
           Word::Bits64(value) => write!(f, "0x{:0bit64_width$x}", value),
        }
    }
}
