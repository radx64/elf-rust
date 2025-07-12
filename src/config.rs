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

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_config_should_return_path_to_inspected_binary() {
        let params = [String::from("bin_name"), String::from("inspected_binary"), String::from("random_stuff")];

        let result = Config::build(params.into_iter()).expect("Should yield proper result config");
        assert_eq!(result.binary_name, "inspected_binary");
    } 

    #[test]
    fn test_config_should_return_error_when_there_is_not_enough_args() {
        let params = [String::from("bin_name")];

        let result = Config::build(params.into_iter());
        assert!(result.is_err());
    }
}
