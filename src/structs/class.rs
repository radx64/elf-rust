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

use std::fmt;

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match &self {
            Class::ELF32 => "Elf32",
            Class::ELF64 => "Elf64",
        })
    }
}
