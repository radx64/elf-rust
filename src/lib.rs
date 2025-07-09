

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
struct ElfHeader{
    signature: [u8; 4],
    class: u8,
    endianness: u8,
    version: u8,
}

fn read_n <const N: usize, I>(iter: &mut I) -> Option<[u8; N]>
where 
    I: Iterator<Item = u8>,
{
    let mut result: [u8; N] = [0; N];
    for i in 0..N {
        result[i] = iter.next()?;
    }
    Some(result)
}

impl ElfHeader {
    fn build<'a>(mut payload: impl Iterator<Item = u8>) -> Result<ElfHeader, &'static str> {
        let sig: [u8; 4] = match read_n::<4, _>(&mut payload) {
            Some(bytes) => bytes,
            None => return Err("Failed to parse signature")
        };
    
        Ok(ElfHeader{signature: sig, class: 0, endianness: 0, version:0})
    }

    fn print(&self) {
        println!("{:02X} {:02X} {:02X} {:02X}", self.signature[0], self.signature[1], self.signature[2], self.signature[3]);
    }
}

use std::fs;

pub fn analyze(config: &Config) -> Result<(), String> {
    let payload = match fs::read(&config.binary_name) {
        Ok(bytes) => bytes,
        Err(error) => return Err(error.to_string()),
    };

    let header = match ElfHeader::build(payload.iter().copied()) {
        Ok(value) => value,
        Err(error) => return Err(String::from("Failed parsing elf header")),
    }; 

    println!("{:?}", header);

    header.print();

    Ok(())
}
