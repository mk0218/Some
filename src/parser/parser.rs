use std::fs::File;
use std::io;

use super::buffer::*;
use super::token::*;


pub struct Parser;

pub type Reader = io::Lines<io::BufReader<File>>;

impl Parser {
    pub fn parse(lines: Reader) {
        let parser = Parser;

        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(why) => panic!("Couldn't read file: {}", why),
            };

            parser.read_line(&line);
        }
    }

    fn read_line(&self, line: &str) -> Result<Vec<Token>, IllegalTokenError> {
        let mut buffer = Buffer::new();
        let mut tokens: Vec<Token> = vec![];

        for c in line.chars() {
            if let Ok(tkn) = Token::try_tokenize(&c.to_string()) {
                tokens.push(Token::try_tokenize(&buffer.content)?);
                tokens.push(tkn);
            } else {
                buffer.push(c);
            }
        }

        Ok(tokens)
    }
}