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
    StratusTechnologiesOpenVos,
    ArmEABI,
    Arm,
    Standalone
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
            0x40 => Some(Abi::ArmEABI),
            0x61 => Some(Abi::Arm),
            0xFF => Some(Abi::Standalone),
            _ => None,
        }
    }
}

use std::fmt;

impl fmt::Display for Abi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match &self {
            Abi::SystemV => "Unix - System V",
            Abi::HpUx => "HP/UX",
            Abi::NetBSD => "NetBSD",
            Abi::Linux => "Linux",
            Abi::GnuHurd => "GNU Hurd",
            Abi::Solaris => "Solaris",
            Abi::AixMonterey => "AIX Monterey",
            Abi::IRIX => "SGI Irix",
            Abi::FreeBSD => "FreeBsd",
            Abi::Tru64 => "TRU64",
            Abi::NovelModesto => "Novel Modesto",
            Abi::OpenBSD => "OpenBSD",
            Abi::OpenVMS => "Open VMS",
            Abi::NonStopKernel => "Non Stop Kernel",
            Abi::AROS => "AROS",
            Abi::FenixOS => "Fenix OS",
            Abi::NuxiCloudAbi => "Nuxi Cloud Abi",
            Abi::StratusTechnologiesOpenVos => "Stratus Technologies Open Vos",
            Abi::ArmEABI => "Arm Embeeded ABI",
            Abi::Arm => "Arm",
            Abi::Standalone => "Standalone (embeeded) application",
        })
    }
}
