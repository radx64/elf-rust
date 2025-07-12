#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    EtNone,
    EtRel,
    EtExec,
    EtDyn,
    EtCore,
    EtOs(u16),
    EtProc(u16),
}

const ET_LOOS : u16 = 0xFE00;
const ET_HIOS : u16 = 0xFEFF;

const ET_LOPROC : u16 = 0xFF00;
const ET_HIPROC : u16 = 0xFFFF;

impl Type{
    pub fn from_u16(value: u16) -> Option<Type> {
        match value {
            0x0000 => Some(Type::EtNone),
            0x0001 => Some(Type::EtRel),
            0x0002 => Some(Type::EtExec),
            0x0003 => Some(Type::EtDyn),
            0x0004 => Some(Type::EtCore),
            ET_LOOS..=ET_HIOS => Some(Type::EtOs(value)),
            ET_LOPROC..=ET_HIPROC => Some(Type::EtProc(value)),
            _ => None,
        }
    }
}

use std::fmt;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num: u16 = match self {
            Type::EtOs(num)     => *num,
            Type::EtProc(num)   => *num,
            _ => 0,
        };
        let description =  match self {
            Type::EtNone   => "None",
            Type::EtRel    => "Relocatable file",
            Type::EtExec   => "Executable file",
            Type::EtDyn    => "Shared object file",
            Type::EtCore   => "Core file",
            Type::EtOs(_)     => "OS Specific",
            Type::EtProc(_)   => "Processor specifc",
        };

        if matches!(self, Type::EtOs(_)) || matches!(self, Type::EtProc(_))
        {
            write!(f, "{} ({})", description, num)
        }
        else {
            write!(f, "{}", description)
        }
    }
}
