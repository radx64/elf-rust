use crate::structs::word::Word;
use super::super::types;
use super::super::consts;
use crate::structs::sectionheadertype::SectionHeaderType;
use crate::bits::*;
use crate::structs::sectionheaderflags;

#[derive(Debug)]
pub struct SectionHeaderEntry {
    sh_name: types::Elf32Word,
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

// TODO add section header name fetching from strings section
impl SectionHeaderEntry {
    pub fn print(&self){
        println!("name offset: 0x{:X}", self.sh_name);
        println!("type: {}", self.sh_type);
        println!("flags:\n{}", sectionheaderflags::flags_to_string(self.sh_flags.to_u64().unwrap()));
        println!("load address: 0x{:X}", self.sh_addr.to_u64().unwrap());
        println!("offset in the image: {} (bytes)", self.sh_offset.to_u64().unwrap());
        println!("size: {} (bytes)", self.sh_size.to_u64().unwrap());
        println!("link: 0x{:X}", self.sh_link);
        println!("info: 0x{:X}", self.sh_info);
        println!("addres alignment: 0x{:X}", self.sh_addralign.to_u64().unwrap());
        println!("entry size: {} (bytes)", self.sh_entsize.to_u64().unwrap());
    }
}

pub struct SectionHeaderInfo{
    pub offset: Word,
    pub entries: types::Elf32Half,
    pub size: types::Elf32Half,
}

#[derive(Debug)]
pub struct SectionHeader{
    entries: Vec<SectionHeaderEntry>
}

impl SectionHeader {
    pub fn build(payload: &[u8], info: &SectionHeaderInfo, is_32bit: bool, is_little_endian: bool) -> Result<SectionHeader, &'static str> {
        if payload.len() < consts::SHSTRNDX64_END { // TODO: add proper validation later
            return Err("Failed to parse. Header too short");
        }

        let mut entries: Vec<SectionHeaderEntry> = Vec::new();
        let first_entry_offset: u64 = info.offset.to_u64()?;

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
                let sh_addralign_offset = entry_offset + 0x20;
                sh_addralign = Word::Bits64(to_u64_from_slice(&payload[sh_addralign_offset..sh_addralign_offset+8].try_into().unwrap(), is_little_endian));
                let sh_entsize_offset = entry_offset + 0x24;
                sh_entsize = Word::Bits64(to_u64_from_slice(&payload[sh_entsize_offset..sh_entsize_offset+8].try_into().unwrap(), is_little_endian));
            }

            let sh_link = types::Elf32Word::from_ne_bytes(payload[sh_link_offset..sh_link_offset+4].try_into().unwrap());
            let sh_info = types::Elf32Word::from_ne_bytes(payload[sh_info_offset..sh_info_offset+4].try_into().unwrap());

            entries.push(SectionHeaderEntry{
                sh_name: sh_name,
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

         Ok(SectionHeader{entries: entries})
    }

    pub fn print(&self){
        println!("Section header segments:");

        for (index, entry) in self.entries.iter().enumerate() {
            println!("");
            println!("Index: {index}");
            entry.print();
        }
 
    }
}
