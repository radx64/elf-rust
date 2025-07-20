use std::collections::HashSet;

pub struct Config {
     arguments : HashSet<Argument>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Argument {
    ShowHelp,
    NoColors,
    ShowElfHeader,
    ShowProgramHeaders,
    ShowSectionHeaders,
    InspectedBinary(String),
    ThisBinary(String),
}

impl Config {
    pub fn should_show_help(&self) -> bool {
        self.arguments.contains(&Argument::ShowHelp)
    }

    pub fn should_disable_colors(&self) -> bool {
        self.arguments.contains(&Argument::NoColors)
    }

    // TODO: try to write a macro or somethig to generalize accesing enum inner values
    pub fn get_own_name(&self) -> &String {
        let binary_name = self.arguments.iter().find_map(|arg| {
            if let Argument::ThisBinary(s) = arg {
                Some(s) 
            } else {
                None
            }
        });
        binary_name.expect("Own binary name expected to be valid")
    }

    pub fn get_inspected_binary_name(&self) -> &String {
        let binary_name = self.arguments.iter().find_map(|arg| {
            if let Argument::InspectedBinary(s) = arg {
                Some(s) 
            } else {
                None
            }
        });
        binary_name.expect("Inspected Binary name expected to be valid")
    }

    pub fn should_display_elf_header(&self) -> bool {
        self.arguments.contains(&Argument::ShowElfHeader) || self.are_all_sections_turned_off()
    }

    pub fn should_display_program_headers(&self) -> bool {
        self.arguments.contains(&Argument::ShowProgramHeaders) || self.are_all_sections_turned_off()
    }
    
    pub fn should_display_section_headers(&self) -> bool {
        self.arguments.contains(&Argument::ShowSectionHeaders) || self.are_all_sections_turned_off()
    }

    fn are_all_sections_turned_off(&self) -> bool {
        !self.arguments.contains(&Argument::ShowElfHeader) && !self.arguments.contains(&Argument::ShowProgramHeaders) && !self.arguments.contains(&Argument::ShowSectionHeaders)
    }

    pub fn print_help(&self) {
        println!("Usage: {} inspected_binary <options>", self.get_own_name());
        println!("  Options are:");
        println!("    -h --help\t\t\tDisplay this help");
        println!("    -c --no-colors\t\tDisable color output");
        println!("    -e --elf-header\t\tDisplay ELF header");
        println!("    -p --program-headers\tDisplay program headers");
        println!("    -s --section-headers\tDisplay section headers");
    }

    fn process_argument(parameter: String) -> Result<Argument, String> {
        if !parameter.starts_with("-") {
            return Ok(Argument::InspectedBinary(parameter))
        }

        if parameter == "-c" || parameter == "--no-colors" {
            return Ok(Argument::NoColors)
        }

        if parameter == "-e" || parameter == "--elf-header" {
            return Ok(Argument::ShowElfHeader)
        } 

        if parameter == "-p" || parameter == "--program-headers" {
            return Ok(Argument::ShowProgramHeaders)
        } 

        if parameter == "-s" || parameter == "--section-headers" {
            return Ok(Argument::ShowSectionHeaders)
        }

        if parameter == "-h" || parameter == "--help" {
            return Ok(Argument::ShowHelp)
        } 

        Err(format!("Unrecognized argument: {}", parameter))
    }

    pub fn build (mut args: impl Iterator<Item = String>) -> Result<Config, String> {

        let mut processed_arguments = HashSet::new();

        processed_arguments.insert(Argument::ThisBinary(match args.next() {
            Some(binary_name) => binary_name,
            None => return Err(String::from("This binary name is invalid."))
        }));

        loop {
            let argument = args.next();

            match argument {
                Some(arg) => { 
                    match Self::process_argument(arg) {
                        Ok(a) => processed_arguments.insert(a),
                        Err(error) => return Err(error),
                    };
                },
                None => { break; }
            }
        };

        if processed_arguments.len() < 2
        {
            return Err(String::from("Not enough arguments."))
        }

        Ok(Config{arguments: processed_arguments})
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_should_return_path_to_inspected_binary() {
        let params = [String::from("bin_name"), String::from("inspected_binary")];

        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert_eq!(result.get_inspected_binary_name(), "inspected_binary");
    } 

    #[test]
    fn test_config_should_return_error_when_there_is_not_enough_args() {
        let params = [String::from("bin_name")];

        let result = Config::build(params.into_iter());
        assert!(result.is_err());
    }

    #[test]
    fn test_config_should_support_help_parameters() {
        let params = [String::from("bin_name"), String::from("-h")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(result.should_show_help());

        let params = [String::from("bin_name"), String::from("--help")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(result.should_show_help());

        let params = [String::from("bin_name"), String::from("")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(!result.should_show_help());
    }
    #[test]
    fn test_config_should_support_elf_header_parameters() {
        let params = [String::from("bin_name"), String::from("-e")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(result.should_display_elf_header());

        let params = [String::from("bin_name"), String::from("--elf-header")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(result.should_display_elf_header());

        let params = [String::from("bin_name"), String::from("")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(result.should_display_elf_header());

        let params = [String::from("bin_name"), String::from("-p")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(!result.should_display_elf_header());
    }

    #[test]
    fn test_config_should_support_program_headers_parameters() {
        let params = [String::from("bin_name"), String::from("-p")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(result.should_display_program_headers());

        let params = [String::from("bin_name"), String::from("--program-headers")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(result.should_display_program_headers());

        let params = [String::from("bin_name"), String::from("")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(result.should_display_program_headers());

        let params = [String::from("bin_name"), String::from("-e")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(!result.should_display_program_headers());
    }

    #[test]
    fn test_config_should_support_section_headers_parameters() {
        let params = [String::from("bin_name"), String::from("-s")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(result.should_display_section_headers());

        let params = [String::from("bin_name"), String::from("--section-headers")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(result.should_display_section_headers());

        let params = [String::from("bin_name"), String::from("")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(result.should_display_section_headers());

        let params = [String::from("bin_name"), String::from("-p")];
        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert!(!result.should_display_section_headers());
    }

}
