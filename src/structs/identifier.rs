use super::class::Class;
use super::endianess::Endianness;
use super::abi::Abi;

use super::super::consts;

#[derive(Debug)]
pub struct Identifier{
    pub magic: [u8; consts::EI_MAG3 + 1],
    pub class:  Class,
    pub endianess: Endianness,
    pub verison: u8,
    pub abi: Abi,
    pub abiversion: u8,
    pub padding: [u8; consts::IDENT_PADDING_BYTES]
}

impl Identifier{
    pub fn build(payload: &[u8])  -> Result<Identifier, &'static str> {
        Ok(Identifier{
            magic: [
                u8::from(payload[consts::EI_MAG0]),
                u8::from(payload[consts::EI_MAG1]),
                u8::from(payload[consts::EI_MAG2]),
                u8::from(payload[consts::EI_MAG3]),
            ],
            class: Class::from_u8(payload[consts::EI_CLASS]).ok_or("Invalid class")?,
            endianess: Endianness::from_u8(payload[consts::EI_DATA]).ok_or("Invalid endianness")?,
            verison: payload[consts::EI_VERSION],
            abi: Abi::from_u8(payload[consts::EI_OSABI]).ok_or("Invalid target ABI")?,
            abiversion: payload[consts::EI_ABIVERSION],
            padding: [0; consts::IDENT_PADDING_BYTES],
        })
    }

    pub fn is32_bit(&self) -> bool {
        self.class == Class::ELF32
    }

    pub fn is_little_endian(&self) -> bool {
        self.endianess == Endianness::LITTLE
    }
}
