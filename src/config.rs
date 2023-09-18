pub struct Config {
    pub path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("No file path provided.");
        }

        Ok(Config { path: args[0].clone() })
    }
}