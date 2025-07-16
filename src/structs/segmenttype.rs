#[derive(Debug)]
pub enum SegmentType {
    PtNull,
    PtLoad,
    PtDynamic,
    PtInterp,
    PtNote,
    PtShlib,
    PtPhdr,
    PtTls,
    PtOs(u32),
    PtProc(u32),
}

const PT_LOOS: u32 = 0x60000000;
const PT_HIOS: u32 = 0x6FFFFFFF;
const PT_LOPROC: u32 = 0x70000000;
const PT_HIPROC: u32 = 0x7FFFFFFF;

impl SegmentType {
    pub fn from_u32(value: &[u8;4], is_little_endian: bool) -> Option<SegmentType> {
        let value = to_u32_from_slice(value, is_little_endian);    // Todo: check if segment type desctipion use endianness (rather not)

        match value {
            0x00000000 => Some(SegmentType::PtNull),
            0x00000001 => Some(SegmentType::PtLoad),
            0x00000002 => Some(SegmentType::PtDynamic),
            0x00000003 => Some(SegmentType::PtInterp),
            0x00000004 => Some(SegmentType::PtNote),
            0x00000005 => Some(SegmentType::PtShlib),
            0x00000006 => Some(SegmentType::PtPhdr),
            0x00000007 => Some(SegmentType::PtTls),
            PT_LOOS..=PT_HIOS => Some(SegmentType::PtOs(value)),
            PT_LOPROC..=PT_HIPROC => Some(SegmentType::PtProc(value)),
            _ => None,
        }
    }
}

use std::fmt;

use crate::bits::to_u32_from_slice;

impl fmt::Display for SegmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num: u32 = match self {
            SegmentType::PtOs(num)     => *num,
            SegmentType::PtProc(num)   => *num,
            _ => 0,
        };
        let description =  match self {
            SegmentType::PtNull     => "Program header table entry unused",
            SegmentType::PtLoad     => "Loadable segment",
            SegmentType::PtDynamic  => "Dynamic linking information",
            SegmentType::PtInterp   => "Interpreter information",
            SegmentType::PtNote     => "Auxiliary information",
            SegmentType::PtShlib    => "Reserved",
            SegmentType::PtPhdr     => "Segment containing program header table itself",
            SegmentType::PtTls      => "Thread-Local Storage template",
            SegmentType::PtOs(_)    => "OS (OS specific)",
            SegmentType::PtProc(_)  => "PROC (Proc specific)",
        };        

        if matches!(self, SegmentType::PtOs(_)) || matches!(self, SegmentType::PtProc(_))
        {
            write!(f, "{} ({})", description, num)
        }
        else {
            write!(f, "{}", description)
        }

    }
}
