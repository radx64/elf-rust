use crate::structs::offset::Offset;

use super::segmenttype::SegmentType;
use super::super::consts;
use super::super::types;

#[derive(Debug)]
pub struct ProgramHeaderEntry {
    p_type: SegmentType,
}

pub struct ProgramHeaderInfo{
    pub offset: Offset,
    pub entries: types::Elf32Half,
    pub size: types::Elf32Half,
}

#[derive(Debug)]
pub struct ProgramHeader{
    entries: Vec<ProgramHeaderEntry>
}

impl ProgramHeader {
    pub fn build(payload: &[u8], info: &ProgramHeaderInfo, is_32bit: bool, is_little_endian: bool) -> Result<ProgramHeader, &'static str> {
        if payload.len() < consts::SHSTRNDX64_END { // TODO: add proper validation later
            return Err("Failed to parse. Header too short");
        }

        let mut entries: Vec<ProgramHeaderEntry> = Vec::new();
        let first_entry_offset: u64 = info.offset.to_u64()?;
        
        const FIELD_SIZE : usize = 4;

        for i in 0..info.entries {
            let entry_offset = first_entry_offset + info.size as u64 * i as u64;
            let entry_offset = entry_offset as usize;

            let p_type = SegmentType::from_u32(&payload[entry_offset..entry_offset+FIELD_SIZE].try_into().unwrap(), is_little_endian).expect("Tried to get segment type");

            let entry = ProgramHeaderEntry{p_type : p_type};
            entries.push(entry);
        }

        Ok(ProgramHeader{entries: entries})
    }

    pub fn print(&self) {
        println!("Program Header:");
        println!("\tSegment type: \t\t{:#?}", &self.entries);
    }
}
