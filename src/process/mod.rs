pub mod register_keys;
pub mod get_tokens;
pub mod build_parsers;
pub mod build_ast;
pub mod codegen_ast;
pub mod codegen_parsers;
pub mod codegen_visitor;

use std::fs::File;
use std::io::Write;
use elapsed::measure_time;
use process::register_keys::RegisterKeys;
use process::get_tokens::GetTokens;
use process::build_parsers::BuildParsers;
use process::build_ast::BuildAst;
use process::codegen_ast::CodegenAst;
use process::codegen_parsers::CodegenParsers;
use process::codegen_visitor::CodegenVisitor;
use ast;
use process;
use lang_data::data::*;
use visit_ast::*;
use std::path::Path;

pub fn process<'a, 'b, 'c>(res: &'a ast::Source, data: &'b mut LangData<'a>, path: &'c str) {
    let (elapsed, ()) = measure_time(|| {
        {
            let mut register_keys = RegisterKeys::new(data);
            let (elapsed, ()) = measure_time(|| {
                register_keys.visit_source(res);
            });
            println!("Register keys: {}", elapsed);
        }
        {
            let mut get_tokens = GetTokens::new(data);
            get_tokens.visit_source(res);
        }
        {
            let mut build_parsers = BuildParsers::new(data);
            build_parsers.visit_source(res);
        }
        {
            let mut build_ast = BuildAst::new(data);
            build_ast.build_ast();
        }
        {
            let codegen_ast = CodegenAst::new(data);
            write_file(path, "ast.rs", codegen_ast.gen());
        }
        {
            let codegen_parsers = CodegenParsers::new(data);
            write_file(path, "parsers.rs", codegen_parsers.gen());
        }
        {
            let codegen_visitor = CodegenVisitor::new(data);
            write_file(path, "visitor.rs", codegen_visitor.gen());
        }
        // Generate mod file
        process::gen_mod(path);
    });
    println!("Process: {}", elapsed);
}

pub fn gen_mod(path: &str) {
    let s = "pub mod ast;\npub mod parsers;\npub mod visitor;".to_string();
    write_file(path, "mod.rs", s);
}

pub fn write_file<'a, 'b>(path: &'a str, name: &'b str, content: String) {
    let p = Path::new(path).join(name);
    let mut file = File::create(p).expect(format!("Could not open path: {}, file: {}", path, name).as_str());
    file.write_all(content.as_bytes()).expect(format!("Could not write to path: {}, file: {}", path, name).as_str());
}