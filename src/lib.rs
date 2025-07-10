mod consts;
mod types;

pub struct Config {
     pub binary_name : String
}

impl Config {
    pub fn build (mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip first argument

        let binary_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Binary name not passed as an argument"),
        };

        Ok(Config{binary_name})
    }
}

#[derive(Debug)]
enum Class {
    ELF32,
    ELF64,
}

impl Class {
    fn from_u8(value: u8) -> Option<Class> {
        match value {
            1 => Some(Class::ELF32),
            2 => Some(Class::ELF64),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Endianness {
    LITTLE,
    BIG,
}

impl Endianness {
    fn from_u8(value: u8) -> Option<Endianness> {
        match value {
            1 => Some(Endianness::LITTLE),
            2 => Some(Endianness::BIG),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Abi {
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
    fn from_u8(value: u8) -> Option<Abi> {
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


#[derive(Debug)]
struct Identifier{
    magic: [char; consts::EI_MAG3 + 1],
    class:  Class,
    endianess: Endianness,
    verison: u8,
    abi: Abi,
    padding: [u8; consts::IDENT_PADDING_BYTES]
}

impl Identifier{
    fn build(payload: &[u8])  -> Result<Identifier, &'static str> {
        Ok(Identifier{
            magic: [
                char::from(payload[consts::EI_MAG0]),
                char::from(payload[consts::EI_MAG1]),
                char::from(payload[consts::EI_MAG2]),
                char::from(payload[consts::EI_MAG3]),
            ],
            class: Class::from_u8(payload[consts::EI_CLASS]).ok_or("Invalid class")?,
            endianess: Endianness::from_u8(payload[consts::EI_DATA]).ok_or("Invalid endianness")?,
            verison: payload[consts::EI_VERSION],
            abi: Abi::from_u8(payload[7]).ok_or("Invalid architecture")?,
            padding: [0; consts::IDENT_PADDING_BYTES]
        })
    }
}

#[derive(Debug)]
enum Type {
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
    fn from_u16(value: u16) -> Option<Type> {
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

#[derive(Debug)]
enum Machine {
    EMNONE, 
    EMM32, 
    EMSPARC, 
    EM386, 
    EM68K, 
    EM88K, 
    EM860, 
    EMMIPS, 
    EMS370, 
    EMMIPSRS3LE,
    EMPARISC,
    EMVPP500,
    EMSPARC32PLUS,
    EM960, 
    EMPPC, 
    EMPPC64, 
    EMS390,
    EMV800, 
    EMFR20, 
    EMRH32, 
    EMRCE, 
    EMARM, 
    EMALPHA, 
    EMSH, 
    EMSPARCV9, 
    EMTRICORE, 
    EMARC, 
    EMH8300, 
    EMH8300H, 
    EMH8S, 
    EMH8500, 
    EMIA64, 
    EMMIPSX, 
    EMCOLDFIRE, 
    EM68HC12, 
    EMMMA, 
    EMPCP, 
    EMNCPU, 
    EMNDR1, 
    EMSTARCORE, 
    EMME16, 
    EMST100, 
    EMTINYJ, 
    EMX8664, 
    EMPDSP, 
    EMPDP10, 
    EMPDP11, 
    EMFX66, 
    EMST9PLUS, 
    EMST7, 
    EM68HC16, 
    EM68HC11, 
    EM68HC08, 
    EM68HC05, 
    EMSVX, 
    EMST19, 
    EMVAX, 
    EMCRIS, 
    EMJAVELIN, 
    EMFIREPATH, 
    EMZSP, 
    EMMMIX, 
    EMHUANY, 
    EMPRISM, 
    EMAVR, 
    EMFR30, 
    EMD10V, 
    EMD30V, 
    EMV850, 
    EMM32R, 
    EMMN10300, 
    EMMN10200, 
    EMPJ, 
    EMOPENRISC, 
    EMARCA5, 
    EMXTENSA, 
    EMVIDEOCORE, 
    EMTMMGPP, 
    EMNS32K, 
    EMTPC, 
    EMSNP1K, 
    EMST200,
    Reserved
}

impl Machine{
    fn from_u16(value: u16) -> Option<Machine> {
        match value {
    0 => Some(Machine::EMNONE),	
    1 => Some(Machine::EMM32),	
    2 => Some(Machine::EMSPARC),	
    3 => Some(Machine::EM386),	
    4 => Some(Machine::EM68K),	
    5 => Some(Machine::EM88K),	
    6 => Some(Machine::Reserved),	
    7 => Some(Machine::EM860),	
    8 => Some(Machine::EMMIPS),	
    9 => Some(Machine::EMS370),	
    10 => Some(Machine::EMMIPSRS3LE),	
    11 => Some(Machine::Reserved),	
    15 => Some(Machine::EMPARISC),	
    16 => Some(Machine::Reserved),	
    17 => Some(Machine::EMVPP500),	
    18 => Some(Machine::EMSPARC32PLUS),	
    19 => Some(Machine::EM960),	
    20 => Some(Machine::EMPPC),	
    21 => Some(Machine::EMPPC64),	
    22 => Some(Machine::EMS390),	
    23 => Some(Machine::Reserved),	
    36 => Some(Machine::EMV800),	
    37 => Some(Machine::EMFR20),	
    38 => Some(Machine::EMRH32),	
    39 => Some(Machine::EMRCE),	
    40 => Some(Machine::EMARM),	
    41 => Some(Machine::EMALPHA),	
    42 => Some(Machine::EMSH),	
    43 => Some(Machine::EMSPARCV9),	
    44 => Some(Machine::EMTRICORE),	
    45 => Some(Machine::EMARC),	
    46 => Some(Machine::EMH8300),	
    47 => Some(Machine::EMH8300H),	
    48 => Some(Machine::EMH8S),	
    49 => Some(Machine::EMH8500),	
    50 => Some(Machine::EMIA64),	
    51 => Some(Machine::EMMIPSX),	
    52 => Some(Machine::EMCOLDFIRE),	
    53 => Some(Machine::EM68HC12),	
    54 => Some(Machine::EMMMA),	
    55 => Some(Machine::EMPCP),	
    56 => Some(Machine::EMNCPU),	
    57 => Some(Machine::EMNDR1),	
    58 => Some(Machine::EMSTARCORE),	
    59 => Some(Machine::EMME16),	
    60 => Some(Machine::EMST100),	
    61 => Some(Machine::EMTINYJ),	
    62 => Some(Machine::EMX8664),	
    63 => Some(Machine::EMPDSP),	
    64 => Some(Machine::EMPDP10),	
    65 => Some(Machine::EMPDP11),	
    66 => Some(Machine::EMFX66),	
    67 => Some(Machine::EMST9PLUS),	
    68 => Some(Machine::EMST7),	
    69 => Some(Machine::EM68HC16),	
    70 => Some(Machine::EM68HC11),	
    71 => Some(Machine::EM68HC08),	
    72 => Some(Machine::EM68HC05),	
    73 => Some(Machine::EMSVX),	
    74 => Some(Machine::EMST19),	
    75 => Some(Machine::EMVAX),	
    76 => Some(Machine::EMCRIS),	
    77 => Some(Machine::EMJAVELIN),	
    78 => Some(Machine::EMFIREPATH),	
    79 => Some(Machine::EMZSP),	
    80 => Some(Machine::EMMMIX),	
    81 => Some(Machine::EMHUANY),	
    82 => Some(Machine::EMPRISM),	
    83 => Some(Machine::EMAVR),	
    84 => Some(Machine::EMFR30),	
    85 => Some(Machine::EMD10V),	
    86 => Some(Machine::EMD30V),	
    87 => Some(Machine::EMV850),	
    88 => Some(Machine::EMM32R),	
    89 => Some(Machine::EMMN10300),	
    90 => Some(Machine::EMMN10200),	
    91 => Some(Machine::EMPJ),	
    92 => Some(Machine::EMOPENRISC),	
    93 => Some(Machine::EMARCA5),	
    94 => Some(Machine::EMXTENSA),	
    95 => Some(Machine::EMVIDEOCORE),	
    96 => Some(Machine::EMTMMGPP),	
    97 => Some(Machine::EMNS32K),	
    98 => Some(Machine::EMTPC),	
    99 => Some(Machine::EMSNP1K),	
    100 => Some(Machine::EMST200),	
            _ => None,
        }
    }
}

// TODO: 
// Need to implement support for both LE and BE?
fn to_u16(h: u8, l: u8) -> u16{
    ((h as u16) << 8) | (l as u16)
}

fn to_u32(hh: u8, hl: u8, lh: u8, ll: u8) -> u32{
    ((hh as u32) << 24) | ((hl as u32) << 16) | ((lh as u32) << 8) | (ll as u32)
}

fn to_u32_from_slice(a: &[u8; 4]) -> u32 {
    to_u32(a[3], a[2], a[1], a[0])
}

fn to_u64_from_slice(a: &[u8; 8]) -> u64 {
    ((to_u32(a[7], a[6], a[5], a[4]) as u64) << 32) | (to_u32(a[3], a[2], a[1], a[0]) as u64)
}

fn to_u64_from_slices(h: &[u8; 4], l:&[u8; 4]) -> u64 {
    ((to_u32(h[3], h[2], h[1], h[0]) as u64) << 32) | (to_u32(l[3], l[2], l[1], l[0]) as u64)
}

#[derive(Debug)]
enum EntryPoint{
    Bits32(types::Elf32Addr),
    Bits64(types::Elf64Addr),
}

impl EntryPoint{
    fn build32(pointer: types::Elf32Addr) -> EntryPoint {
        EntryPoint::Bits32(pointer)
    }
    fn build64(pointer: types::Elf64Addr) -> EntryPoint {
        EntryPoint::Bits64(pointer)
    }

    fn build(h: &[u8;4], l: &[u8;4], is32bit: bool) -> EntryPoint {
        match is32bit {
            true => EntryPoint::build32(to_u32_from_slice(l)),
            false => EntryPoint::build64(to_u64_from_slices(h, l)),
        }

    }
}

#[derive(Debug)]
struct ElfHeader{
    e_ident: Identifier,
    e_type: Type,
    e_machine: Machine,
    e_verison: u32,
    e_entry: EntryPoint
}

// fn read_n <const N: usize, I>(iter: &mut I) -> Option<[u8; N]>
// where 
//     I: Iterator<Item = u8>,
// {
//     let mut result: [u8; N] = [0; N];
//     for i in 0..N {
//         result[i] = iter.next()?;
//     }
//     Some(result)
// }

impl ElfHeader {
    fn build(payload: &[u8]) -> Result<ElfHeader, &'static str> {
        if payload.len() < 4 {
            return Err("Failed to parse. Header too short");
        }

        let identifier = match Identifier::build(&payload) {
            Ok(identifier) => identifier,
            Err(error) => return Err(error),
        };

        let is32Bit = identifier.endianess == Endianness::LITTLE;

        Ok(ElfHeader{
            e_ident: identifier,
            e_type: Type::from_u16(to_u16(payload[0x11], payload[0x10])).ok_or("Invalid type")?,
            e_machine: Machine::from_u16(to_u16(payload[0x13], payload[0x12])).ok_or("Invalid machine type")?,
            e_verison: to_u32(payload[0x17], payload[0x15], payload[0x15], payload[0x14]),
            e_entry: EntryPoint::build(&payload[0x22..0x26].try_into().expect("msg"), &payload[0x18..0x1C].try_into().expect("msg2"), is32Bit), // todo, get rid of .expect()

            // TODO: ElfHeader have 32 and 64 bit versions and next field (e_entry) differs in size (first field using addresses)
            // Need to think how to handle that, traits, enums?
            // POC with enum seems to work, need to polish it a bit
        })
    }
}
 
use std::fs;

pub fn analyze(config: &Config) -> Result<(), String> {
    let payload = match fs::read(&config.binary_name) {
        Ok(bytes) => bytes,
        Err(error) => return Err(error.to_string()),
    };

    let header = match ElfHeader::build(&payload) {
        Ok(value) => value,
        Err(error) => return Err(format!("Failed parsing elf header due to: {}", error)),
    }; 
    println!("{:#?}", header);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_header_build() {
        let payload = vec![0x7F, 0x45, 0x4C, 0x46, 0x00, 0x00, 0x00];
        let header = ElfHeader::build(&payload);
        assert!(header.is_ok());
    }

    #[test]
    fn test_elf_header_build_too_short() {
        let payload = vec![0x7F, 0x45, 0x4C];
        let header = ElfHeader::build(&payload);
        assert!(header.is_err());
    }
}
