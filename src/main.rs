extern crate nom;
extern crate elapsed;
use elapsed::measure_time;
#[macro_use]
extern crate descr_common;
extern crate descr_lang;
mod lang_data;
mod process;
use lang_data::data::*;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("\n = Missing args ========================= ");
        eprintln!("|                                        |");
        eprintln!("|   Usage: <exe> input-file output-dir   |");
        eprintln!("|                                        |");
        eprintln!(" ======================================== \n");
        return;
    }
    let filename = args[1].as_str();
    let output_dir = args[2].as_str();
    let mut f = File::open(filename).expect(format!("Could not open {}", filename).as_str());
    let mut buf = Vec::with_capacity(1024);
    f.read_to_end(&mut buf).expect(format!("Could not open {}", filename).as_str());
    let (elapsed, res) = measure_time(|| {
        descr_lang::gen::parsers::source(&buf[..])
    });
    println!("Parse: {}", elapsed);
    //println!("{:#?}", res);
    let mut data = LangData::new();
    {
        match res {
            nom::IResult::Done(_, ref source) => {
                process::process(source, &mut data, output_dir);
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
