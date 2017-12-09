#[macro_use]
extern crate descr_common;
mod lang;
extern crate nom;
extern crate json_descr;

fn main() {
    let buf = descr_common::util::load_file("../pg-example.pg");
    let res = lang::parsers::start(&buf[..]);
    println!("\n= Result ===========================");
    println!("{:#?}", res);
    let r = json_descr::lang::parsers::start(b"{
        \"test\": [1, 2, 3],
        \"test2\": { \"test3\": 1 }
    }");
    println!("{:#?}", r);
    match r {
        nom::IResult::Done(i, ref o) => {
            let src = String::new();
            let src = json_descr::lang::to_source::ToSource::to_source_js_object(src, o);
            println!("{}", src);
        },
        _ => {}
    }
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