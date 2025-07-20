use std::fs;

pub mod config;
pub mod termcolors; 
mod bits;
mod consts;
mod types;
mod structs;

use structs::elfheader::ElfHeader;
use config::Config;

use crate::structs::{programheader::{ProgramHeader, ProgramHeaderInfo}, sectionheader::{SectionHeader, SectionHeaderInfo}};

pub fn analyze(config: &Config) -> Result<(), String> {
    let payload = match fs::read(config.get_inspected_binary_name()) {
        Ok(bytes) => bytes,
        Err(error) => return Err(error.to_string()),
    };

    let elf_header = match ElfHeader::build(&payload) {
        Ok(value) => value,
        Err(error) => return Err(format!("Failed parsing elf header due to \"{}\"", error)),
    }; 

    if config.should_display_elf_header() {
        elf_header.print();
        println!("");
    }

    let is_32bit = elf_header.is32_bit();
    let is_little_endian = elf_header.is_little_endian();

    let program_header_info= ProgramHeaderInfo {
        offset: elf_header.program_header_offset(),
        entries: elf_header.program_header_entries(),
        size: elf_header.program_header_size()};
        
    let program_header = match ProgramHeader::build(&payload, &program_header_info, is_32bit, is_little_endian) {
        Ok(value) => value,
        Err(error) => return Err(format!("Failed parsing program header due to \"{}\"", error)),
    };

    if config.should_display_program_headers() {
        program_header.print();
        println!("");
    }
    
    let section_header_info = SectionHeaderInfo {
        offset: elf_header.section_header_offset(),
        entries: elf_header.section_header_entries(),
        size: elf_header.section_header_size(),
        names_index: elf_header.section_names_index()};

    let section_header = match SectionHeader::build(&payload, &section_header_info, is_32bit, is_little_endian) {
        Ok(value) => value,
        Err(error) => return Err(format!("Failed parsing section header due to \"{}\"", error)),
    };

    if config.should_display_section_headers() {
        section_header.print();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_header_build_too_short() {
        let payload = vec![0x7F, 0x45, 0x4C];
        let header = ElfHeader::build(&payload);
        assert!(header.is_err());
    }

    #[test]
    fn test_program_header_build_too_short() {
        let payload = vec![0x7F, 0x45, 0x4C];
        let info = ProgramHeaderInfo {offset: structs::word::Word::Bits64(0x0), entries: 10, size: 500};
        let header = ProgramHeader::build(&payload, &info, false, false);
        assert!(header.is_err());
    }

    #[test]
    fn test_section_header_build_too_short() {
        let payload = vec![0x7F, 0x45, 0x4C];
        let info = SectionHeaderInfo {offset: structs::word::Word::Bits64(0x0), entries: 10, size: 500, names_index: 10};
        let header = SectionHeader::build(&payload, &info, false, false);
        assert!(header.is_err());
    }
}
