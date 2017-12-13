pub mod register_keys;
pub mod get_tokens;
pub mod build_parsers;
pub mod build_ast;
pub mod codegen_ast;
pub mod codegen_parsers;
pub mod codegen_visitor;
pub mod codegen_tosource;
pub mod codegen_syntax;

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
use self::codegen_tosource::CodegenToSource;
use self::codegen_syntax::CodegenSyntax;
use descr_lang::gen::ast;
use lang_data::data::*;
use std::path::Path;
use descr_lang::gen::visitor::Visitor;

pub fn process<'a : 'd, 'b, 'c, 'd>(res: &'a ast::Source, data: &'d mut LangData<'d>, path: &'c str) {
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
        {
            measure!("Codegen tosource", {
                let codegen_tosource = CodegenToSource::new(data);
                write_file(path, "to_source.rs", codegen_tosource.gen());
            });
        }
        {
            measure!("Codegen syntax", {
                let codegen_syntax = CodegenSyntax::new(data);
                codegen_syntax.gen();
                //write_file(path, "to_source.rs", codegen_tosource.gen());
            });
        }
        // Generate mod file
        gen_mod(path, data);
    });
    println!("Process: {}", elapsed);
}

pub fn gen_mod<'a, 'd>(path: &str, _data: &'a LangData<'d>) {
    let s = "pub mod ast;\npub mod parsers;\npub mod visitor;\npub mod to_source;\n\n".to_string();
    /*
    use std::collections::HashSet;
    let start_key = data.start_key.expect("Start key not found");
    let start_part = data.typed_parts
                .get(start_key)
                .expect(&format!("Could not find token for start key: {}", start_key));
    let needs_lifetime = start_part.needs_lifetime(data, &mut HashSet::new());
    s += "extern crate descr_common;\n";
    s += "extern crate nom;\n";
    s += "use std::result::Result;\n";
    s += "use self::ast::";
    s += data.rule_types
        .get(start_key)
        .expect(&format!("Coult not get ast {}", start_key))
        .get_type_name(data);
    s += ";\n";
    // Parse file
    s += "pub fn parse_file";
    if needs_lifetime { s += "<'a>"; }
    s += "(filename: &str) -> Result<";
    s = start_part.add_type(s, data);
    s += ", &str> {\n";
    s += "    let buf = descr_common::util::load_file(filename);\n";
    s += "    parse_bytes(&buf[..])\n";
    s += "}\n\n";
    // Parse str
    s += "pub fn parse_str";
    if needs_lifetime { s += "<'a>"; }
    s += "(string: &str) -> Result<";
    s = start_part.add_type(s, data);
    s += ", &str> {\n";
    s += "    parse_bytes(string.as_bytes())\n";
    s += "}\n\n";
    // Parse bytes
    s += "pub fn parse_bytes";
    if needs_lifetime { s += "<'a>"; }
    s += "(bytes: &[u8]) -> Result<";
    s = start_part.add_type(s, data);
    s += ", &str> {\n";
    s += "    let res = self::parsers::start(bytes);\n";
    s += "    match res {\n";
    s += "        nom::IResult::Done(i, o) => Result::Ok(o),\n";
    s += "        nom::IResult::Error(ref e) => Result::Err(e.description()),\n";
    s += "    }\n";
    s += "}\n\n";
    */
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
