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
