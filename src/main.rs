extern crate elapsed;
extern crate nom;
use elapsed::measure_time;
extern crate descr_common;
extern crate descr_lang;
extern crate descr_gen;
extern crate pg;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use descr_gen::lang_data::data::LangData;
use std::path::Path;

fn invalid_args() {
    eprintln!("\n = Missing args ================================ ");
    eprintln!("|                                               |");
    eprintln!("|   Usage: <exe> input-file output-dir          |");
    eprintln!("|          - Process lang-file to output dir    |");
    eprintln!("|   or                                          |");
    eprintln!("|          <exe> descr-lang                     |");
    eprintln!("|          - Process descr lang                 |");
    eprintln!("|   or                                          |");
    eprintln!("|          <exe> pg-lang                        |");
    eprintln!("|          - Process playground lang            |");
    eprintln!("|   or                                          |");
    eprintln!("|          <exe> pg                             |");
    eprintln!("|          - Playground result, will check      |");
    eprintln!("|            for changes                        |");
    eprintln!("|                                               |");
    eprintln!(" =============================================== \n");
}

enum Command {
    Pg,
    PgRes,
    DescrLang,
    Custom
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (filename, output_dir, command, check_change) = match args.len() {
        2 => match args[1].as_str() {
            "pg-lang"     => ("pg/pg.lang", "pg/src/lang", Command::Pg, false),
            "pg"          => ("pg/pg.lang", "pg/src/lang", Command::PgRes, true),
            "descr-lang"  => ("descr.lang", "descr-lang/src/gen", Command::DescrLang, false),
            _ => {
                invalid_args();
                return;
            }
        },
        3 => (args[1].as_str(), args[2].as_str(), Command::Custom, false),
        _ => {
            invalid_args();
            return;
        }
    };
    let file_path = Path::new(filename);
    // Check modification of lang file vs ast.rs
    let ast_path = Path::new(output_dir).join("ast.rs");
    let is_changed = if ast_path.exists() {
        let file_meta = std::fs::metadata(file_path).unwrap();
        let ast_meta = std::fs::metadata(ast_path).unwrap();
        match file_meta.modified() {
            Ok(file_time) => match ast_meta.modified() {
                Ok(ast_time) => file_time > ast_time,
                _ => true
            },
            _ => true
        }
    } else {
        true
    };
    if check_change && !is_changed {
        println!("Lang file not changes since last codegen");
    } else {
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
                    descr_gen::process::process(source, &mut data, output_dir);
                }
                _ => ()
            }
        }
    }
    match command {
        Command::Pg => println!("Playground lang updated"),
        Command::PgRes => {
            if is_changed {
                println!("Playground lang updated, reload to see result");
            } else {
                pg::result();
            }
        },
        Command::DescrLang => println!("Descr lang updated"),
        Command::Custom => println!("Langfile processed")
    };

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
