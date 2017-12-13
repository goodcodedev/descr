//#[macro_use]
extern crate descr_common;
mod lang;
extern crate nom;
extern crate json_descr;

fn main() {
    let buf = descr_common::util::load_file("../pg-example.pg");
    let res = lang::parsers::start(&buf[..]);
    println!("\n= Result ===========================");
    println!("{:#?}", res);
    /*
    match res {
        nom::IResult::Done(i, ref o) => {
            let src = String::new();
            let src = lang::to_source::ToSource::to_source_source(src, o);
            println!("{}", src);
        },
        _ => {}
    }
    */
}