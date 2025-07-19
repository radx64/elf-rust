use crate::structs::word::Word;
use crate::types;
use crate::structs::sectionheadertype::SectionHeaderType;
use crate::bits::*;
use crate::structs::sectionheaderflags;

#[derive(Debug)]
pub struct SectionHeaderEntry {
    sh_name: types::Elf32Word,
    sh_name_str: String,
    sh_type: SectionHeaderType,
    sh_flags: Word,
    sh_addr: Word,
    sh_offset: Word,
    sh_size: Word,
    sh_link: types::Elf32Word,
    sh_info: types::Elf32Word,
    sh_addralign: Word,
    sh_entsize: Word,
}

impl SectionHeaderEntry {
    pub fn print(&self){
        println!("\t0x{:08X}\t0x{:08X}\t0x{:08X}\t0x{:08X}\t{:16}",
            self.sh_addr.to_u64().unwrap(),
            self.sh_offset.to_u64().unwrap(),
            self.sh_link,

            self.sh_entsize.to_u64().unwrap(),
            self.sh_name_str,
        );
        println!("\t0x{:08X}\t\t\t0x{:08X}\t0x{:08X}\t{}",
            self.sh_addralign.to_u64().unwrap(), 
            self.sh_info,
            self.sh_size.to_u64().unwrap(),
            self.sh_type);

        let flags = self.sh_flags.to_u64().unwrap();

        if flags != 0 {
            println!("{}", sectionheaderflags::flags_to_string(self.sh_flags.to_u64().unwrap()));
        } else {
            println!("\t[No flags]");
        }
    }
}

pub struct SectionHeaderInfo{
    pub offset: Word,
    pub entries: types::Elf32Half,
    pub size: types::Elf32Half,
    pub names_index: types::Elf32Half,
}

#[derive(Debug)]
pub struct SectionHeader{
    entries: Vec<SectionHeaderEntry>,
}

impl SectionHeader {
    pub fn build(payload: &[u8], info: &SectionHeaderInfo, is_32bit: bool, is_little_endian: bool) -> Result<SectionHeader, &'static str> {
        if payload.len() < info.offset.to_u64().unwrap() as usize + info.size as usize { 
            return Err("Section header too short");
        }

        let mut entries: Vec<SectionHeaderEntry> = Vec::new();
        let first_entry_offset: u64 = info.offset.to_u64()?;

        let mut string_section_offset = 0;

        for i in 0..info.entries {
            let entry_offset = (first_entry_offset + info.size as u64 * i as u64) as usize;
            let sh_name = types::Elf32Word::from_ne_bytes(payload[entry_offset..entry_offset+4].try_into().unwrap());
            let sh_type_offset = entry_offset + 0x04;
            let sh_type = SectionHeaderType::from_u32(&payload[sh_type_offset..sh_type_offset+4].try_into().unwrap(), is_little_endian).expect("Tried to get section header type");

            let sh_flags_offset = entry_offset + 0x08;
            let sh_flags;
            let sh_addr;
            let sh_offset;
            let sh_size;

            let sh_link_offset;
            let sh_info_offset;

            let sh_addralign;
            let sh_entsize;

            if is_32bit {
                sh_flags = Word::Bits32(to_u32_from_slice(&payload[sh_flags_offset..sh_flags_offset+4].try_into().unwrap(), is_little_endian));
                let sh_addr_offset = entry_offset + 0x0C;
                sh_addr = Word::Bits32(to_u32_from_slice(&payload[sh_addr_offset..sh_addr_offset+4].try_into().unwrap(), is_little_endian));
                let sh_offset_offset = entry_offset + 0x10;
                sh_offset = Word::Bits32(to_u32_from_slice(&payload[sh_offset_offset..sh_offset_offset+4].try_into().unwrap(), is_little_endian));
                let sh_size_offset =  entry_offset + 0x14;
                sh_size = Word::Bits32(to_u32_from_slice(&payload[sh_size_offset..sh_size_offset+4].try_into().unwrap(), is_little_endian));
                sh_link_offset = entry_offset + 0x18;
                sh_info_offset = entry_offset + 0x1C;
                let sh_addralign_offset = entry_offset + 0x20;
                sh_addralign = Word::Bits32(to_u32_from_slice(&payload[sh_addralign_offset..sh_addralign_offset+4].try_into().unwrap(), is_little_endian));
                let sh_entsize_offset = entry_offset + 0x24;
                sh_entsize = Word::Bits32(to_u32_from_slice(&payload[sh_entsize_offset..sh_entsize_offset+4].try_into().unwrap(), is_little_endian));

            } else {
                sh_flags = Word::Bits64(to_u64_from_slice(&payload[sh_flags_offset..sh_flags_offset+8].try_into().unwrap(), is_little_endian));
                let sh_addr_offset = entry_offset + 0x10;
                sh_addr = Word::Bits64(to_u64_from_slice(&payload[sh_addr_offset..sh_addr_offset+8].try_into().unwrap(), is_little_endian));
                let sh_offset_offset = entry_offset + 0x18;
                sh_offset = Word::Bits64(to_u64_from_slice(&payload[sh_offset_offset..sh_offset_offset+8].try_into().unwrap(), is_little_endian));
                let sh_size_offset =  entry_offset + 0x20;
                sh_size = Word::Bits64(to_u64_from_slice(&payload[sh_size_offset..sh_size_offset+8].try_into().unwrap(), is_little_endian));
                sh_link_offset = entry_offset + 0x28;
                sh_info_offset = entry_offset + 0x2C;
                let sh_addralign_offset = entry_offset + 0x30;
                sh_addralign = Word::Bits64(to_u64_from_slice(&payload[sh_addralign_offset..sh_addralign_offset+8].try_into().unwrap(), is_little_endian));
                let sh_entsize_offset = entry_offset + 0x38;
                sh_entsize = Word::Bits64(to_u64_from_slice(&payload[sh_entsize_offset..sh_entsize_offset+8].try_into().unwrap(), is_little_endian));
            }

            let sh_link = types::Elf32Word::from_ne_bytes(payload[sh_link_offset..sh_link_offset+4].try_into().unwrap());
            let sh_info = types::Elf32Word::from_ne_bytes(payload[sh_info_offset..sh_info_offset+4].try_into().unwrap());

            let sh_name_str = String::from("UNKNOWN_NAME");

            if let SectionHeaderType::ShtStrtab = sh_type {
                if i == info.names_index {
                    string_section_offset = sh_offset.to_u64().unwrap() as usize;
                }
            }

            entries.push(SectionHeaderEntry{
                sh_name: sh_name,
                sh_name_str: sh_name_str,
                sh_type: sh_type, 
                sh_flags: sh_flags,
                sh_addr: sh_addr,
                sh_offset: sh_offset,
                sh_size: sh_size,
                sh_link: sh_link,
                sh_info: sh_info,
                sh_addralign: sh_addralign,
                sh_entsize: sh_entsize});
        }

        for entry in entries.iter_mut(){

            let offset = string_section_offset + entry.sh_name as usize;
            entry.sh_name_str = string_until_null(payload[offset..].to_vec());
        }

         Ok(SectionHeader{entries: entries})
    }

    pub fn print(&self){
        println!("Section header segments:");
 
        println!("[Idx]\tLoadAddr\tImgOffset\tLink\t\tEntrySize\tName");
        println!("\tAlignment\t\t\tInfo\t\tSize\t\tType");
        println!("\tFlags");
        for (index, entry) in self.entries.iter().enumerate() {
            print!("[{index:3}]");
            entry.print();
            println!("");
        }
 
    }
}

fn string_until_null(vec: Vec<u8>) -> String {
    let slice = match vec.iter().position(|&b| b == 0) {
        Some(pos) => &vec[..pos],
        None => &vec[..],
    };

    String::from_utf8_lossy(slice).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_unitl_null_should_read_to_first_null() {
        let payload: Vec<u8> = vec![0x52, 0x41, 0x44, 0x00];
        let result = string_until_null(payload);
        assert_eq!(result, String::from("RAD"));
    }

    #[test]
    fn string_unitl_null_should_hadle_empty_payload() {
        let payload: Vec<u8> = vec![];
        let result = string_until_null(payload);
        assert_eq!(result, String::from(""));
    }

    #[test]
    fn string_unitl_null_should_hadle_only_first_null() {
        let payload: Vec<u8> = vec![0x72, 0x61, 0x64, 0x00, 0x52, 0x41, 0x44, 0x00];
        let result = string_until_null(payload);
        assert_eq!(result, String::from("rad"));
    }

    #[test]
    fn string_unitl_null_should_hadle_only_nulls() {
        let payload: Vec<u8> = vec![0x00, 0x00, 0x00];
        let result = string_until_null(payload);
        assert_eq!(result, String::from(""));
    }
}
