#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

pub mod xml;
pub mod xmlrpc;

