#[derive(Debug)]
pub enum Type {
    EtNone,
    EtRel,
    EtExec,
    EtDyn,
    EtCore,
    EtLoos,
    EtHios,
    EtLoProc,
    EtHiProc,
}

impl Type{
    pub fn from_u16(value: u16) -> Option<Type> {
        match value {
            0x0000 => Some(Type::EtNone),
            0x0001 => Some(Type::EtRel),
            0x0002 => Some(Type::EtExec),
            0x0003 => Some(Type::EtDyn),
            0x0004 => Some(Type::EtCore),
            0xFE00 => Some(Type::EtLoos),
            0xFEFF => Some(Type::EtHios),
            0xFF00 => Some(Type::EtLoProc),
            0xFFFF => Some(Type::EtHiProc),
            _ => None,
        }
    }
}
