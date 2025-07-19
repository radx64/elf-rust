use std::env;
use std::process;

use elf_rust::analyze;
use elf_rust::config::Config;
use elf_rust::termcolors;

#[allow(dead_code)]
fn main() {
    termcolors::enable_colors();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}Problem parsing arguments: {err}", termcolors::red());
        process::exit(1);
    });

    println!("Analyzing: {}{}{} ...", termcolors::green(), config.binary_name, termcolors::default());
    let result = analyze(&config).unwrap_or_else(|err| {
        eprintln!("{}Problem analyzing payload: {err}", termcolors::red());
        process::exit(1);
    });
    result
}
