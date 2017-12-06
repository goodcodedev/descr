pub mod register_keys;
pub mod get_tokens;
pub mod build_parsers;
pub mod build_ast;
pub mod codegen_ast;
pub mod codegen_parsers;
pub mod codegen_visitor;

use std::fs::File;
use std::fs;
use std::io::Write;
use elapsed::measure_time;
use self::register_keys::RegisterKeys;
use self::get_tokens::GetTokens;
use self::build_parsers::BuildParsers;
use self::build_ast::BuildAst;
use self::codegen_ast::CodegenAst;
use self::codegen_parsers::CodegenParsers;
use self::codegen_visitor::CodegenVisitor;
use descr_lang::gen::ast;
use lang_data::data::*;
use std::path::Path;
use descr_lang::gen::visitor::Visitor;

pub fn process<'a, 'b, 'c>(res: &'a ast::Source, data: &'b mut LangData<'a>, path: &'c str) {
    let (elapsed, ()) = measure_time(|| {
        {
            measure!("Register keys", {
                let mut register_keys = RegisterKeys::new(data);
                register_keys.visit_source(res);
            });
        }
        {
            measure!("Get tokens", {
                let mut get_tokens = GetTokens::new(data);
                get_tokens.visit_source(res);
            });
        }
        {
            measure!("Build parsers", {
                let mut build_parsers = BuildParsers::new(data);
                build_parsers.visit_source(res);
            });
        }
        {
            measure!("Build ast", {
                let mut build_ast = BuildAst::new(data);
                build_ast.build_ast();
            })
        }
        {
            measure!("Codegen ast", {
                let codegen_ast = CodegenAst::new(data);
                write_file(path, "ast.rs", codegen_ast.gen());
            });
        }
        {
            measure!("Codegen parsers", {
                let codegen_parsers = CodegenParsers::new(data);
                write_file(path, "parsers.rs", codegen_parsers.gen());
            });
        }
        {
            measure!("Codegen visitor", {
                let codegen_visitor = CodegenVisitor::new(data);
                write_file(path, "visitor.rs", codegen_visitor.gen());
            });
        }
        // Generate mod file
        gen_mod(path);
    });
    println!("Process: {}", elapsed);
}

pub fn gen_mod(path: &str) {
    let s = "pub mod ast;\npub mod parsers;\npub mod visitor;".to_string();
    write_file(path, "mod.rs", s);
}

pub fn write_file<'a, 'b>(path: &'a str, name: &'b str, content: String) {
    let p = Path::new(path).join(name);
    if p.exists() {
        // Backup file (for now)
        let backup_p = Path::new(path).join("backup").join(name);
        fs::copy(&p, backup_p).expect("Could not backup file");
    }
    let mut file =
        File::create(p).expect(format!("Could not open path: {}, file: {}", path, name).as_str());
    file.write_all(content.as_bytes())
        .expect(format!("Could not write to path: {}, file: {}", path, name).as_str());
}
