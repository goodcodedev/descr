mod lang;
extern crate descr_common;

fn main() {
    let buf = descr_common::util::load_file("example.pg");
    let res = lang::parsers::start(&buf[..]);
    println!("\n= Result ============");
    println!("{:#?}", res);
}
