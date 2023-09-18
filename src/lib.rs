pub mod config;
pub mod syntax;
pub mod parser;

use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

use config::Config;
use parser::parse;


pub fn run(config: Config) -> Result<(), &'static str> {
    let lines = match read_lines(&config.path) {
        Ok(lines) => lines,
        Err(why) => panic!("Couldn't open file: {}", why),
    };
    parse(lines);
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
