pub mod config;

mod bits;
mod consts;
mod types;
mod structs;

use structs::elfheader::ElfHeader;

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

use std::fs;
use config::Config;

use crate::structs::{programheader::{ProgramHeader, ProgramHeaderInfo}, sectionheader::{SectionHeader, SectionHeaderInfo}};

pub fn analyze(config: &Config) -> Result<(), String> {
    let payload = match fs::read(&config.binary_name) {
        Ok(bytes) => bytes,
        Err(error) => return Err(error.to_string()),
    };

    let elf_header = match ElfHeader::build(&payload) {
        Ok(value) => value,
        Err(error) => return Err(format!("Failed parsing elf header due to: {}", error)),
    }; 
    elf_header.print();

    let is_32bit = elf_header.is32_bit();
    let is_little_endian = elf_header.is_little_endian();

    let program_header_info= ProgramHeaderInfo {
        offset: elf_header.program_header_offset(),
        entries: elf_header.program_header_entries(),
        size: elf_header.program_header_size()};
        
    let program_header = match ProgramHeader::build(&payload, &program_header_info, is_32bit, is_little_endian) {
        Ok(value) => value,
        Err(error) => return Err(format!("Failed parsing program header due to: {}", error)),
    };

    program_header.print();

    let section_header_info = SectionHeaderInfo {
        offset: elf_header.section_header_offset(),
        entries: elf_header.section_header_entries(),
        size: elf_header.section_header_size(),
        names_index: elf_header.section_names_index()};

    let section_header = match SectionHeader::build(&payload, &section_header_info, is_32bit, is_little_endian) {
        Ok(value) => value,
        Err(error) => return Err(format!("Failed parsing section header due to: {}", error)),
    };

    section_header.print();

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
