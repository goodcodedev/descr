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
use visit_ast::*;
use lang_data::data::*;
use process::register_keys::RegisterKeys;
use process::get_tokens::GetTokens;
use process::build_parsers::BuildParsers;
use process::build_ast::BuildAst;
use process::codegen_ast::CodegenAst;
use process::codegen_parsers::CodegenParsers;
use process::codegen_visitor::CodegenVisitor;

fn main() {
    let input = b"
Source (items:sourceItems)

sourceItems[] WS SourceItem
SourceItem {
    AstSingle,
    AstMany,
    List
}

AstSingle(ident LPAREN tokens:tokenList RPAREN)
AstMany(ident LBRACE items:astItems RBRACE)

tokenList[] WS Token
Token {
    TokenKey(ident optional:QUESTION?),
    TokenNamedKey(name:ident COLON key:ident optional:QUESTION?)
}

astItems[] COMMA AstItem
AstItem {
    AstDef(ident? LPAREN tokens:tokenList RPAREN),
    AstRef(ident)
}
List {
    ListSingle(ident sep:ident reference:ident),
    ListMany(ident sep:ident? LBRACE items:listItems RBRACE)
}
listItems[] COMMA ListItem
ListItem(ident AstItem sep:ident?)

    ";
    let (elapsed, res) = measure_time(|| {
        source(input)
    });
    println!("Parse: {}", elapsed);
    //println!("{:#?}", res);
    let mut data = LangData::new();
    {
        match res {
            nom::IResult::Done(_, ref source) => process(source, &mut data),
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
    println!("{:#?}", data.type_refs);
}

fn process<'a, 'b>(res: &'a ast::Source, data: &'b mut LangData<'a>) {
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
            codegen_ast.gen();
        }
        {
            let codegen_parsers = CodegenParsers::new(data);
            codegen_parsers.gen();
        }
        {
            let codegen_visitor = CodegenVisitor::new(data);
            codegen_visitor.gen();
        }
    });
    println!("Process: {}", elapsed);
}