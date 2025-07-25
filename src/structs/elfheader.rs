use super::identifier::Identifier;
use super::machine::Machine;
use super::type_::Type;
use super::word::Word;

use crate::termcolors::*;
use crate::types;
use crate::consts;
use crate::bits::*;

#[derive(Debug)]
pub struct ElfHeader{
    e_ident: Identifier,
    e_type: Type,
    e_machine: Machine,
    e_verison: types::Elf32Word,
    e_entry: Word,
    e_phoff: Word,
    e_shoff: Word,
    e_flags: types::Elf32Word,
    e_ehsize: types::Elf32Half,
    e_phentsize: types::Elf32Half,
    e_phnum: types::Elf32Half,
    e_shentsize: types::Elf32Half,
    e_shnum: types::Elf32Half,
    e_shstrndx: types::Elf32Half,
}

impl ElfHeader {
    pub fn build(payload: &[u8]) -> Result<ElfHeader, &'static str> {
        if payload.len() < consts::SHSTRNDX64_END { // TODO: add proper validation later
            return Err("Elf header too short");
        }

        let identifier = match Identifier::build(&payload) {
            Ok(identifier) => identifier,
            Err(error) => return Err(error),
        };

        let is32_bit = identifier.is32_bit();
        let is_little_endian = identifier.is_little_endian();

        let program_header_offset;
        let section_header_table_offset;
        let flags;
        let header_size;
        let program_header_table_size;
        let program_header_table_entries;
        let section_header_table_size;
        let section_header_table_entries;
        let section_header_table_names_index;

        if is32_bit {
            program_header_offset = Word::build32(&payload[consts::PHOFF32_START..consts::PHOFF32_END].try_into().unwrap(),  is_little_endian);
            section_header_table_offset = Word::build32(&payload[consts::SHOFF32_START..consts::SHOFF32_END].try_into().unwrap(),  is_little_endian);
            flags = to_u32_from_slice(&payload[consts::FLAGS32_START..consts::FLAGS32_END].try_into().unwrap(), is_little_endian);
            header_size = to_u16_from_slice(&payload[consts::HEADERSIZE32_START..consts::HEADERSIZE32_END].try_into().unwrap(), is_little_endian);
            program_header_table_size = to_u16_from_slice(&payload[consts::PHENTSIZE32_START..consts::PHENTSIZE32_END].try_into().unwrap(), is_little_endian);
            program_header_table_entries = to_u16_from_slice(&payload[consts::PHNUM32_START..consts::PHNUM32_END].try_into().unwrap(), is_little_endian);
            section_header_table_size = to_u16_from_slice(&payload[consts::SHENTSIZE32_START..consts::SHENTSIZE32_END].try_into().unwrap(), is_little_endian);
            section_header_table_entries = to_u16_from_slice(&payload[consts::SHENTNUM32_START..consts::SHENTNUM32_END].try_into().unwrap(), is_little_endian);
            section_header_table_names_index = to_u16_from_slice(&payload[consts::SHSTRNDX32_START..consts::SHSTRNDX32_END].try_into().unwrap(), is_little_endian);

        } else {
            program_header_offset = Word::build64(&payload[consts::PHOFF64_START..consts::PHOFF64_END].try_into().unwrap(), is_little_endian);
            section_header_table_offset = Word::build64(&payload[consts::SHOFF64_START..consts::SHOFF64_END].try_into().unwrap(), is_little_endian);
            flags = to_u32_from_slice(&payload[consts::FLAGS64_START..consts::FLAGS64_END].try_into().unwrap(), is_little_endian);
            header_size = to_u16_from_slice(&payload[consts::HEADERSIZE64_START..consts::HEADERSIZE64_END].try_into().unwrap(), is_little_endian);
            program_header_table_size = to_u16_from_slice(&payload[consts::PHENTSIZE64_START..consts::PHENTSIZE64_END].try_into().unwrap(), is_little_endian);
            program_header_table_entries = to_u16_from_slice(&payload[consts::PHNUM64_START..consts::PHNUM64_END].try_into().unwrap(), is_little_endian);
            section_header_table_size = to_u16_from_slice(&payload[consts::SHENTSIZE64_START..consts::SHENTSIZE64_END].try_into().unwrap(), is_little_endian);
            section_header_table_entries = to_u16_from_slice(&payload[consts::SHENTNUM64_START..consts::SHENTNUM64_END].try_into().unwrap(), is_little_endian);
            section_header_table_names_index = to_u16_from_slice(&payload[consts::SHSTRNDX64_START..consts::SHSTRNDX64_END].try_into().unwrap(), is_little_endian);
        }

        Ok(ElfHeader{
            e_ident: identifier,
            e_type: Type::from_u16(to_u16(payload[0x11], payload[0x10])).ok_or("Invalid type")?,
            e_machine: Machine::from_u16(to_u16(payload[0x13], payload[0x12])).ok_or("Invalid machine type")?,
            e_verison: to_u32(payload[0x17], payload[0x15], payload[0x15], payload[0x14]),
            e_entry: Word::build(&payload[0x22..0x26].try_into().unwrap(), &payload[0x18..0x1C].try_into().unwrap(), is32_bit, is_little_endian),
            e_phoff: program_header_offset,
            e_shoff: section_header_table_offset,
            e_flags: flags,
            e_ehsize: header_size,
            e_phentsize: program_header_table_size,
            e_phnum: program_header_table_entries,
            e_shentsize: section_header_table_size,
            e_shnum: section_header_table_entries,
            e_shstrndx: section_header_table_names_index,
        })
    }

    pub fn print(&self) {
        println!("{}Elf Header:{}", purple(), default());
        println!("\tIdentification:");
        println!("\t\tMagic:\t\t{}{}{}", green(), print_hex_arr(&self.e_ident.magic), default());
        println!("\t\tClass:\t\t{}{}{}", green(), self.e_ident.class, default());
        println!("\t\tEndianness:\t{}{}{}", green(), self.e_ident.endianess, default());
        println!("\t\tVersion:\t{}{:?}{}", green(), self.e_ident.verison, default());
        println!("\t\tAbi:\t\t{}{}{}", green(), self.e_ident.abi, default());
        println!("\t\tAbiVersion:\t{}{:?}{}", green(), self.e_ident.abiversion, default());
        println!("\t\tPadding:\t{}{}{}", gray(), print_hex_arr(&self.e_ident.padding), default());
        println!("\tType:\t\t\t\t{}{}{}", green(), &self.e_type, default());
        println!("\tMachine:\t\t\t{}{}{}", green(), &self.e_machine, default());
        println!("\tVersion:\t\t\t{}{}{}", green(), &self.e_verison, default());
        println!("\tEntry point:\t\t\t{}{:x}{}", green(), &self.e_entry, default());
        println!("\tProgram header:\t\t\t{}{:x}{} (offset)", green(), &self.e_phoff, default());
        println!("\tSection header:\t\t\t{}{:x}{} (offset)", green(), &self.e_shoff, default());
        println!("\tFlags:\t\t\t\t{}{:04X}{}", gray(), &self.e_flags, default());
        println!("\tElf header size:\t\t{}{}{} (bytes)", green(), &self.e_ehsize, default());
        println!("\tProgram header entry size:\t{}{}{} (bytes)", green(), &self.e_phentsize, default());
        println!("\tProgram header entries:\t\t{}{}{}", green(), &self.e_phnum, default());
        println!("\tSection header size:\t\t{}{}{} (bytes)", green(), &self.e_shentsize, default());
        println!("\tSection header entries:\t\t{}{}{}", green(), &self.e_shnum, default());
        println!("\tSection names index:\t\t{}{}{}", green(), &self.e_shstrndx, default());
    }

    pub fn is32_bit(&self) -> bool {
        self.e_ident.is32_bit()
    }

    pub fn program_header_offset(&self) -> Word {
        self.e_phoff
    }

    pub fn program_header_entries(&self) -> types::Elf32Half{
        self.e_phnum
    }

    pub fn program_header_size(&self) -> types::Elf32Half{
        self.e_phentsize
    }

    pub fn is_little_endian(&self) -> bool {
        self.e_ident.is_little_endian()
    }

    pub fn section_header_offset(&self) -> Word {
        self.e_shoff
    }

    pub fn section_header_entries(&self) -> types::Elf32Half {
        self.e_shnum
    }

    pub fn section_header_size(&self) -> types::Elf32Half {
        self.e_shentsize
    }

    pub fn section_names_index(&self) -> types::Elf32Half {
        self.e_shstrndx
    }


}
 
use std::fmt::UpperHex;
fn print_hex_arr<T, const N: usize>(bytes: &[T; N]) -> String
where
    T : UpperHex
{
    let mut result = String::new();
    for byte in bytes {
        result += &format!("{:02X} ", byte);
    }
    result
}
