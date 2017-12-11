use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
#[allow(unused_imports)]
use std;
use super::ast::*;

named!(pub start<Source>, do_parse!(res: source >> (res)));

named!(pub ast_single<AstSingle>,
    do_parse!(
        sp >> ident_k: ident >>
        sp >> char!('(') >>
        sp >> tokens_k: token_list >>
        sp >> char!(')') >>
        (AstSingle {
            ident: ident_k,
            tokens: tokens_k,
        }))
);

named!(pub source<Source>,
    do_parse!(
        sp >> items_k: source_items >>
        (Source {
            items: items_k,
        }))
);

named!(pub token_type<TokenType>, alt_complete!(
    do_parse!(
        sp >> key_k: ident >>
        (TokenType::KeyTokenItem(KeyToken {
            key: key_k,
        })))
    | do_parse!(
        sp >> string_k: quoted_str >>
        (TokenType::QuotedItem(Quoted {
            string: string_k,
        })))
));

named!(pub source_items<Vec<SourceItem>>, many0!(
    map!(ast_single, |node| { SourceItem::AstSingleItem(node) })
));

named!(pub token_list<Vec<Token>>, many0!(alt_complete!(
    do_parse!(
        sp >> name_k: ident >>
        sp >> char!(':') >>
        not_k: opt!(do_parse!(sp >> res: char!('!') >> (res))) >>
        sp >> token_type_k: token_type >>
        optional_k: opt!(do_parse!(sp >> res: char!('?') >> (res))) >>
        (Token::NamedTokenItem(NamedToken {
            name: name_k,
            not: not_k.is_some(),
            token_type: token_type_k,
            optional: optional_k.is_some(),
        })))
    | do_parse!(
        not_k: opt!(do_parse!(sp >> res: char!('!') >> (res))) >>
        sp >> token_type_k: token_type >>
        optional_k: opt!(do_parse!(sp >> res: char!('?') >> (res))) >>
        (Token::SimpleTokenItem(SimpleToken {
            not: not_k.is_some(),
            token_type: token_type_k,
            optional: optional_k.is_some(),
        })))
    | do_parse!(
        not_k: opt!(do_parse!(sp >> res: char!('!') >> (res))) >>
        sp >> char!('(') >>
        sp >> token_list_k: token_list >>
        sp >> char!(')') >>
        optional_k: opt!(do_parse!(sp >> res: char!('?') >> (res))) >>
        (Token::TokenGroupItem(TokenGroup {
            not: not_k.is_some(),
            token_list: token_list_k,
            optional: optional_k.is_some(),
        })))
)));

