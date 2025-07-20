use std::env;
use std::process;

use elf_rust::analyze;
use elf_rust::config::Config;
use elf_rust::termcolors;

#[allow(dead_code)]
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}Problem parsing arguments: {err}", termcolors::red());
        process::exit(1);
    });

    if config.should_disable_colors() {
        termcolors::disable_colors();
    } else {
        termcolors::enable_colors();
    }

    if config.should_show_help(){
        config.print_help();
        process::exit(0);
    }

    println!("Analyzing: {}{}{} ...", termcolors::green(), config.get_inspected_binary_name(), termcolors::default());
    let result = analyze(&config).unwrap_or_else(|err| {
        eprintln!("{}Problem analyzing payload: {err}", termcolors::red());
        process::exit(1);
    });
    result
}
