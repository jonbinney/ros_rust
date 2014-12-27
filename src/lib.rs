#![feature(slicing_syntax)]
#![feature(phase)]

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

#[phase(plugin, link)]
extern crate log;

pub mod xml;
pub mod http;
pub mod xmlrpc;

