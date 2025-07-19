use crate::bits::*;
use crate::structs::word::Word;
use crate::types;

use super::segmentflags;
use super::segmenttype::SegmentType;

#[derive(Debug)]
pub struct ProgramHeaderEntry {
    p_type: SegmentType,
    p_flags: types::Elf32Word,
    p_offset: Word,
    p_vaddr: Word,
    p_paddr: Word,
    p_filesz: Word,
    p_memsz: Word,
    p_align: Word,
}

impl ProgramHeaderEntry {
    pub fn print(&self) {
        println!("{:8x}\t{:8x}\t{:8x}\t{:8x}\t{:8x}\t{:4}\t{:8x}\t{}",
            self.p_offset,
            self.p_vaddr,
            self.p_paddr,
            self.p_filesz,
            self.p_memsz,
            segmentflags::flags_to_string(self.p_flags),
            self.p_align,
            self.p_type);
    }
}

pub struct ProgramHeaderInfo{
    pub offset: Word,
    pub entries: types::Elf32Half,
    pub size: types::Elf32Half,
}

#[derive(Debug)]
pub struct ProgramHeader{
    entries: Vec<ProgramHeaderEntry>
}

impl ProgramHeader {
    pub fn build(payload: &[u8], info: &ProgramHeaderInfo, is_32bit: bool, is_little_endian: bool) -> Result<ProgramHeader, &'static str> {
        if payload.len() < info.offset.to_u64().unwrap() as usize + info.size as usize { 
            return Err("Program header too short");
        }

        let mut entries: Vec<ProgramHeaderEntry> = Vec::new();
        let first_entry_offset: u64 = info.offset.to_u64()?;
        
        const FIELD_SIZE : usize = 4;

        for i in 0..info.entries {
            let entry_offset = (first_entry_offset + info.size as u64 * i as u64) as usize;

            let p_type = SegmentType::from_u32(&payload[entry_offset..entry_offset+FIELD_SIZE].try_into().unwrap(), is_little_endian).expect("Tried to get segment type");
            let p_flags;
            let p_offset;
            let p_vaddr;
            let p_paddr;
            let p_filesz;
            let p_memsz;
            let p_align;

            if is_32bit {
                let p_flags_offset = entry_offset + 0x18;
                p_flags = to_u32_from_slice(&payload[p_flags_offset..p_flags_offset+4].try_into().unwrap(), is_little_endian);
                let p_offset_offset = entry_offset + 0x04;
                p_offset = Word::Bits32(to_u32_from_slice(&payload[p_offset_offset..p_offset_offset+4].try_into().unwrap(), is_little_endian));
                let p_vaddr_offset = entry_offset + 0x08;
                p_vaddr = Word::Bits32(to_u32_from_slice(&payload[p_vaddr_offset..p_vaddr_offset+4].try_into().unwrap(), is_little_endian));
                let p_paddr_offset = entry_offset + 0x0C;
                p_paddr = Word::Bits32(to_u32_from_slice(&payload[p_paddr_offset..p_paddr_offset+4].try_into().unwrap(), is_little_endian));
                let p_filesz_offset = entry_offset + 0x10;
                p_filesz = Word::Bits32(to_u32_from_slice(&payload[p_filesz_offset..p_filesz_offset+4].try_into().unwrap(), is_little_endian));
                let p_memsz_offset = entry_offset + 0x14;
                p_memsz = Word::Bits32(to_u32_from_slice(&payload[p_memsz_offset..p_memsz_offset+4].try_into().unwrap(), is_little_endian));
                let p_align_offset = entry_offset + 0x1C;
                p_align = Word::Bits32(to_u32_from_slice(&payload[p_align_offset..p_align_offset+4].try_into().unwrap(), is_little_endian));

            } else {
                let p_flags_offset = entry_offset + 0x4;
                p_flags = to_u32_from_slice(&payload[p_flags_offset..p_flags_offset+4].try_into().unwrap(), is_little_endian);
                let p_offset_offset = entry_offset + 0x08;
                p_offset = Word::Bits64(to_u64_from_slice(&payload[p_offset_offset..p_offset_offset+8].try_into().unwrap(), is_little_endian));
                let p_vaddr_offset = entry_offset + 0x10;
                p_vaddr = Word::Bits64(to_u64_from_slice(&payload[p_vaddr_offset..p_vaddr_offset+8].try_into().unwrap(), is_little_endian));
                let p_paddr_offset = entry_offset + 0x18;
                p_paddr = Word::Bits64(to_u64_from_slice(&payload[p_paddr_offset..p_paddr_offset+8].try_into().unwrap(), is_little_endian));
                let p_filesz_offset = entry_offset + 0x20;
                p_filesz = Word::Bits64(to_u64_from_slice(&payload[p_filesz_offset..p_filesz_offset+8].try_into().unwrap(), is_little_endian));
                let p_memsz_offset = entry_offset + 0x28;
                p_memsz = Word::Bits64(to_u64_from_slice(&payload[p_memsz_offset..p_memsz_offset+8].try_into().unwrap(), is_little_endian));
                let p_align_offset = entry_offset + 0x30;
                p_align = Word::Bits64(to_u64_from_slice(&payload[p_align_offset..p_align_offset+8].try_into().unwrap(), is_little_endian));
            }

            let entry = ProgramHeaderEntry{
                p_type: p_type,
                p_flags: p_flags,
                p_offset: p_offset,
                p_vaddr: p_vaddr,
                p_paddr: p_paddr,
                p_filesz: p_filesz,
                p_memsz: p_memsz,
                p_align: p_align};

            entries.push(entry);
        }

        Ok(ProgramHeader{entries: entries})
    }

    pub fn print(&self) {
        println!("Program header segments:");
        println!("Idx\tOffset\t\tVirtAddr\tPhysAddr\tFileSiz\t\tMemSiz\t\tFlags\tAlignment\tType");
        for (index, entry) in self.entries.iter().enumerate() {
            print!("{index:3}\t");
            entry.print();
        }
    }
}
