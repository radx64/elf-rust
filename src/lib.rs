pub mod config;

mod bits;
use bits::*;

mod consts;
mod types;

mod structs;
use structs::identifier::Identifier;
use structs::machine::Machine;
use structs::type_::Type;
use structs::entrypoint::EntryPoint;

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

        let is32_bit = identifier.is32_bit();

        Ok(ElfHeader{
            e_ident: identifier,
            e_type: Type::from_u16(to_u16(payload[0x11], payload[0x10])).ok_or("Invalid type")?,
            e_machine: Machine::from_u16(to_u16(payload[0x13], payload[0x12])).ok_or("Invalid machine type")?,
            e_verison: to_u32(payload[0x17], payload[0x15], payload[0x15], payload[0x14]),
            e_entry: EntryPoint::build(&payload[0x22..0x26].try_into().expect("msg"), &payload[0x18..0x1C].try_into().expect("msg2"), is32_bit), // todo, get rid of .expect()

            // TODO: ElfHeader have 32 and 64 bit versions and next field (e_entry) differs in size (first field using addresses)
            // Need to think how to handle that, traits, enums?
            // POC with enum seems to work, need to polish it a bit
        })
    }
}
 
use std::fs;
use config::Config;

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
    // fn test_elf_header_build() {
    //     let payload = vec![0x7F, 0x45, 0x4C, 0x46, 0x00, 0x00, 0x00];
    //     let header = ElfHeader::build(&payload);
    //     assert!(header.is_ok());
    // }

    #[test]
    fn test_elf_header_build_too_short() {
        let payload = vec![0x7F, 0x45, 0x4C];
        let header = ElfHeader::build(&payload);
        assert!(header.is_err());
    }
}
