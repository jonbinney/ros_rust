#![feature(slicing_syntax)]
#![feature(io)]

#![feature(plugin)]
#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;

#[macro_use]
extern crate log;

pub mod xml;
pub mod http;
pub mod xmlrpc;

