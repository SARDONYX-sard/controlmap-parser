//! # ControlMap Parser
//!
//! `controlmap_parser` is a minimal parser for controlmap.txt files.
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;

pub use parser::parse;
