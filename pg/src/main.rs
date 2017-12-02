#[macro_use]
extern crate descr_common;
mod lang;
extern crate nom;

fn main() {
    let buf = descr_common::util::load_file("../pg-example.pg");
    let res = lang::parsers::start(&buf[..]);
    println!("\n= Result ===========================");
    println!("{:#?}", res);
}