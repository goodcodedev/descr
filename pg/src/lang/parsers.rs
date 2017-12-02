use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use std;
use super::ast::*;

named!(pub start<Container>, do_parse!(res: container >> (res)));

named!(pub ast_name<AstName>,
    do_parse!(
        sp >> char!('(') >>
        sp >> ident_k: ident >>
        sp >> char!(')') >>
        (AstName {
            ident: ident_k,
        }))
);

named!(pub container<Container>,
    do_parse!(
        sp >> ident_k: ident >>
        sp >> char!(':') >>
        sp >> ast_name_k: ast_name >>
        (Container {
            ident: ident_k,
            ast_name: ast_name_k,
        }))
);

