//! # ControlMap Parser
//!
//! `controlmap_parser` is a minimal parser for controlmap.txt files.
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;

pub use parser::parse;

#[cfg(test)]
mod parse_test {
    use super::*;

    #[test]
    fn parse_controlmap() {
        let unparsed_file = std::fs::read_to_string("test-files/controlmap_test.txt").expect("Failed to read file.");

        let parsed = parse(&unparsed_file).expect("Failed to parse file.");

        let result = std::fs::read_to_string("test-files/expected.json")
            .expect("Failed to read expected file.");

        assert_eq!(parsed, result);
    }
}
