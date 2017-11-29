#[macro_use]
extern crate nom;
extern crate elapsed;
#[macro_use]
mod util;
use elapsed::measure_time;
mod ast;
mod lang_data;
mod parsers;
mod process;
use parsers::*;
mod visit_ast;
use lang_data::data::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("descr.lang").expect("Could not open descr.lang");
    let mut buf = Vec::with_capacity(1024);
    f.read_to_end(&mut buf).expect("Could not read descr.lang");
    let (elapsed, res) = measure_time(|| {
        source(&buf[..])
    });
    println!("Parse: {}", elapsed);
    //println!("{:#?}", res);
    let mut data = LangData::new();
    {
        match res {
            nom::IResult::Done(_, ref source) => {
                process::process(source, &mut data, "descr-lang/src");
            }
            _ => ()
        }
    }
    /*
    println!("Ast keys: {:#?}", data.ast_data.keys());
    println!("List keys: {:#?}", data.list_data.keys());
    println!("Parts: {:#?}", data.typed_parts.values());
    println!("Ast: {:#?}", data.ast_data.values());
    println!("List: {:#?}", data.list_data.values());
    println!("Structs: {:#?}", data.ast_structs);
    println!("Enums: {:#?}", data.ast_enums);
    */
}
