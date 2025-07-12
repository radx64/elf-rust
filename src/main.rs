use std::env;
use std::process;

use elf_rust::analyze;
use elf_rust::config::Config;

#[allow(dead_code)]
fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Analyzing: {} ...", config.binary_name);

    let result = analyze(&config).unwrap_or_else(|err| {
        eprintln!("Problem analyzing payload: {err}");
        process::exit(1);
    });
    println!();
    println!("Analysis finished");
    result
}
