#[derive(Debug, PartialEq, Eq)]
pub enum Class {
    ELF32,
    ELF64,
}

impl Class {
    pub fn from_u8(value: u8) -> Option<Class> {
        match value {
            1 => Some(Class::ELF32),
            2 => Some(Class::ELF64),
            _ => None,
        }
    }
}
