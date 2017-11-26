#[macro_use]
extern crate nom;
mod ast;
mod lang_data;
mod parsers;
mod process;
use parsers::*;
mod visit_ast;
use visit_ast::*;
use lang_data::*;
use process::register_keys::RegisterKeys;
use process::get_tokens::GetTokens;
use process::build_parsers::BuildParsers;
use process::build_ast::BuildAst;

fn main() {
    let input = b"
Source {
    AstSingle,
    AstMany,
    List
}

AstSingle(ident LPAREN tokenList RPAREN)
AstMany(ident LBRACE astItems RBRACE)

tokenList[] WS Token
Token {
    TokenKey(ident optional:QUESTION?),
    TokenNamedKey(name:ident COLON key:ident optional:QUESTION?)
}

astItems[] COMMA AstItem
AstItem {
    AstDef(ident? LPAREN tokenList RPAREN),
    AstRef(ident)
}
    ";
    let res = source(input);
    println!("{:#?}", res);
    let mut data = LangData::new();
    {
        match res {
            nom::IResult::Done(_, ref source) => process(source, &mut data),
            _ => ()
        }
    }
    println!("Ast keys: {:#?}", data.ast_data.keys());
    println!("List keys: {:#?}", data.list_data.keys());
    println!("Parts: {:#?}", data.typed_parts.values());
    println!("Ast: {:#?}", data.ast_data.values());
    println!("List: {:#?}", data.list_data.values());
    println!("Structs: {:#?}", data.ast_structs);
    println!("Enums: {:#?}", data.ast_enums);
}

fn process<'a, 'b>(res: &'a ast::Source, data: &'b mut LangData<'a>) {
    {
        let mut register_keys = RegisterKeys::new(data);
        register_keys.visit_source(res);
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
}