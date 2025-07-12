#[derive(Debug, PartialEq, Eq)]
pub enum Endianness {
    LITTLE,
    BIG,
}

impl Endianness {
    pub fn from_u8(value: u8) -> Option<Endianness> {
        match value {
            1 => Some(Endianness::LITTLE),
            2 => Some(Endianness::BIG),
            _ => None,
        }
    }
}
