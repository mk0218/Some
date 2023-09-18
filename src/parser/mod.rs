mod buffer;
mod parser;
mod token;

use crate::parser::parser::{ Parser, Reader };




pub fn parse(lines: Reader) {
    Parser::parse(lines);
}