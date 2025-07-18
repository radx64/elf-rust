use crate::bits::to_u32_from_slice;

#[derive(Debug)]
pub enum SectionHeaderType{
    ShtNull,
    ShtProgbits,
    ShtSymtab,
    ShtStrtab,
    ShtRela,
    ShtHash,
    ShtDynamic,
    ShtNote,
    ShtNobits,
    ShtRel,
    ShtShlib,
    ShtDynsym,
    ShtInitArray,
    ShtFiniArray,
    ShtPreinitArray,
    ShtGroup,
    ShtSymtabShndx,
    ShtNum,
    ShtLoos(u32)
}

const SHT_LOOS: u32 = 0x60000000;

impl SectionHeaderType {
    pub fn from_u32(value: &[u8;4], is_little_endian: bool) -> Option<SectionHeaderType>  {
        let value = to_u32_from_slice(value, is_little_endian); // Todo: check if section type description use endianness (rather not)

        match value {
            0x0 => Some(SectionHeaderType::ShtNull),
            0x1 => Some(SectionHeaderType::ShtProgbits),
            0x2 => Some(SectionHeaderType::ShtSymtab),
            0x3 => Some(SectionHeaderType::ShtStrtab),
            0x4 => Some(SectionHeaderType::ShtRela),
            0x5 => Some(SectionHeaderType::ShtHash),
            0x6 => Some(SectionHeaderType::ShtDynamic),
            0x7 => Some(SectionHeaderType::ShtNote),
            0x8 => Some(SectionHeaderType::ShtNobits),
            0x9 => Some(SectionHeaderType::ShtRel),
            0x0A => Some(SectionHeaderType::ShtShlib),
            0x0B => Some(SectionHeaderType::ShtDynsym),
            0x0E => Some(SectionHeaderType::ShtInitArray),
            0x0F => Some(SectionHeaderType::ShtFiniArray),
            0x10 => Some(SectionHeaderType::ShtPreinitArray),
            0x11 => Some(SectionHeaderType::ShtGroup),
            0x12 => Some(SectionHeaderType::ShtSymtabShndx),
            0x13 => Some(SectionHeaderType::ShtNum),
            _ => {
                if value >= SHT_LOOS {
                    Some(SectionHeaderType::ShtLoos(value))
                } else {
                    None
                }
            },
        }
    }

 }

use std::fmt;

impl fmt::Display for SectionHeaderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num: u32 = match self {
            SectionHeaderType::ShtLoos(num)     => *num,
            _ => 0,
        };
        let description =  match self {
            SectionHeaderType::ShtNull          => "(NULL) Section header table entry unused",
            SectionHeaderType::ShtProgbits      => "(PROGBITS) Program data",
            SectionHeaderType::ShtSymtab        => "(SYMTAB) Symbol table",
            SectionHeaderType::ShtStrtab        => "(STRTAB) String table",
            SectionHeaderType::ShtRela          => "(RELA) Relocation entries with addends",
            SectionHeaderType::ShtHash          => "(HASH) Symbol hash table",
            SectionHeaderType::ShtDynamic       => "(DYNAMIC) Dynamic linking information",
            SectionHeaderType::ShtNote          => "(NOTE) Notes",
            SectionHeaderType::ShtNobits        => "(NOBITS) Program space with no data (bss)",
            SectionHeaderType::ShtRel           => "(REL) Relocation entries, no addends",
            SectionHeaderType::ShtShlib         => "(SHLIB) Reserved",
            SectionHeaderType::ShtDynsym        => "(DYNSYM) Dynamic linker symbol table",
            SectionHeaderType::ShtInitArray     => "(INITARRAY) Array of constructors",
            SectionHeaderType::ShtFiniArray     => "(FINIARRAY) Array of destructors",
            SectionHeaderType::ShtPreinitArray  => "(PREINITARRAY) Array of pre-constructors",
            SectionHeaderType::ShtGroup         => "(GROUP) Section group",
            SectionHeaderType::ShtSymtabShndx   => "(SYMTABSHNDX) Extended section indices",
            SectionHeaderType::ShtNum           => "(NUM) Number of defined types",
            SectionHeaderType::ShtLoos(_)       => "(OS) (OS specific)",

        };        

        if matches!(self, SectionHeaderType::ShtLoos(_))
        {
            write!(f, "{} (0x{:X})", description, num)
        }
        else {
            write!(f, "{}", description)
        }

    }
}




// 0x0	SHT_NULL	Section header table entry unused
// 0x1	SHT_PROGBITS	Program data
// 0x2	SHT_SYMTAB	Symbol table
// 0x3	SHT_STRTAB	String table
// 0x4	SHT_RELA	Relocation entries with addends
// 0x5	SHT_HASH	Symbol hash table
// 0x6	SHT_DYNAMIC	Dynamic linking information
// 0x7	SHT_NOTE	Notes
// 0x8	SHT_NOBITS	Program space with no data (bss)
// 0x9	SHT_REL	Relocation entries, no addends
// 0x0A	SHT_SHLIB	Reserved
// 0x0B	SHT_DYNSYM	Dynamic linker symbol table
// 0x0E	SHT_INIT_ARRAY	Array of constructors
// 0x0F	SHT_FINI_ARRAY	Array of destructors
// 0x10	SHT_PREINIT_ARRAY	Array of pre-constructors
// 0x11	SHT_GROUP	Section group
// 0x12	SHT_SYMTAB_SHNDX	Extended section indices
// 0x13	SHT_NUM	Number of defined types.
// 0x60000000	SHT_LOOS	Start OS-specific.
