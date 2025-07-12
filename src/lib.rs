pub mod config;

mod bits;
use bits::*;

mod consts;
mod types;

mod structs;
use structs::identifier::Identifier;
use structs::machine::Machine;
use structs::type_::Type;
use structs::entrypoint::EntryPoint;
use structs::offset::Offset;

// fn read_n <const N: usize, I>(iter: &mut I) -> Option<[u8; N]>
// where 
//     I: Iterator<Item = u8>,
// {
//     let mut result: [u8; N] = [0; N];
//     for i in 0..N {
//         result[i] = iter.next()?;
//     }
//     Some(result)
// }

#[derive(Debug)]
struct ElfHeader{
    e_ident: Identifier,
    e_type: Type,
    e_machine: Machine,
    e_verison: u32,
    e_entry: EntryPoint,
    e_phoff: Offset,
    e_shoff: Offset,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl ElfHeader {
    fn build(payload: &[u8]) -> Result<ElfHeader, &'static str> {
        if payload.len() < consts::SHSTRNDX64_END { // TODO: add proper validation later
            return Err("Failed to parse. Header too short");
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
            program_header_offset = Offset::build32(&payload[consts::PHOFF32_START..consts::PHOFF32_END].try_into().unwrap(),  is_little_endian);
            section_header_table_offset = Offset::build32(&payload[consts::SHOFF32_START..consts::SHOFF32_END].try_into().unwrap(),  is_little_endian);
            flags = to_u32_from_slice(&payload[consts::FLAGS32_START..consts::FLAGS32_END].try_into().unwrap(), is_little_endian);
            header_size = to_u16_from_slice(&payload[consts::HEADERSIZE32_START..consts::HEADERSIZE32_END].try_into().unwrap(), is_little_endian);
            program_header_table_size = to_u16_from_slice(&payload[consts::PHENTSIZE32_START..consts::PHENTSIZE32_END].try_into().unwrap(), is_little_endian);
            program_header_table_entries = to_u16_from_slice(&payload[consts::PHNUM32_START..consts::PHNUM32_END].try_into().unwrap(), is_little_endian);
            section_header_table_size = to_u16_from_slice(&payload[consts::SHENTSIZE32_START..consts::SHENTSIZE32_END].try_into().unwrap(), is_little_endian);
            section_header_table_entries = to_u16_from_slice(&payload[consts::SHENTNUM32_START..consts::SHENTNUM32_END].try_into().unwrap(), is_little_endian);
            section_header_table_names_index = to_u16_from_slice(&payload[consts::SHSTRNDX32_START..consts::SHSTRNDX32_END].try_into().unwrap(), is_little_endian);
        } else {
            program_header_offset = Offset::build64(&payload[consts::PHOFF64_START..consts::PHOFF64_END].try_into().unwrap(), is_little_endian);
            section_header_table_offset = Offset::build64(&payload[consts::SHOFF64_START..consts::SHOFF64_END].try_into().unwrap(), is_little_endian);
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
            e_entry: EntryPoint::build(&payload[0x22..0x26].try_into().unwrap(), &payload[0x18..0x1C].try_into().unwrap(), is32_bit, is_little_endian), // Todo: think what to do with unwraps
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
}
 
use std::fs;
use config::Config;

pub fn analyze(config: &Config) -> Result<(), String> {
    let payload = match fs::read(&config.binary_name) {
        Ok(bytes) => bytes,
        Err(error) => return Err(error.to_string()),
    };

    let header = match ElfHeader::build(&payload) {
        Ok(value) => value,
        Err(error) => return Err(format!("Failed parsing elf header due to: {}", error)),
    }; 
    println!("{:#?}", header);

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    // fn test_elf_header_build() {
    //     let payload = vec![0x7F, 0x45, 0x4C, 0x46, 0x00, 0x00, 0x00];
    //     let header = ElfHeader::build(&payload);
    //     assert!(header.is_ok());
    // }

    #[test]
    fn test_elf_header_build_too_short() {
        let payload = vec![0x7F, 0x45, 0x4C];
        let header = ElfHeader::build(&payload);
        assert!(header.is_err());
    }
}
