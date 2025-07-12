#[derive(Debug)]
pub enum Abi {
    SystemV,
    HpUx,
    NetBSD,
    Linux,
    GnuHurd,
    Solaris,
    AixMonterey,
    IRIX,
    FreeBSD,
    Tru64,
    NovelModesto,
    OpenBSD,
    OpenVMS,
    NonStopKernel,
    AROS,
    FenixOS,
    NuxiCloudAbi,
    StratusTechnologiesOpenVos
}

impl Abi {
    pub fn from_u8(value: u8) -> Option<Abi> {
        match value {
            0x00 => Some(Abi::SystemV),
            0x01 => Some(Abi::HpUx),
            0x02 => Some(Abi::NetBSD),
            0x03 => Some(Abi::Linux),
            0x04 => Some(Abi::GnuHurd),
            0x06 => Some(Abi::Solaris),
            0x07 => Some(Abi::AixMonterey),
            0x08 => Some(Abi::IRIX),
            0x09 => Some(Abi::FreeBSD),
            0x0A => Some(Abi::Tru64),
            0x0B => Some(Abi::NovelModesto),
            0x0C => Some(Abi::OpenBSD),
            0x0D => Some(Abi::OpenVMS),
            0x0E => Some(Abi::NonStopKernel),
            0x0F => Some(Abi::AROS),
            0x10 => Some(Abi::FenixOS),
            0x11 => Some(Abi::NuxiCloudAbi),
            0x12 => Some(Abi::StratusTechnologiesOpenVos),
            _ => None,
        }
    }
}
