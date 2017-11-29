mod lang;
extern crate descr_common;

pub fn result() {
    let buf = descr_common::util::load_file("pg/example.pg");
    let res = lang::parsers::start(&buf[..]);
    println!("\n= Result ============");
    println!("{:#?}", res);
}