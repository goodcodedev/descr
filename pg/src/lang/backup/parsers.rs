use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use std;
use super::ast::*;

named!(pub start<Source>, do_parse!(res: source >> (res)));

named!(pub expr<Expr>, alt_complete!(
    do_parse!(
        sp >> op1_k: debug_wrap!(expr) >>
        sp >> debug_wrap!(tag!("+")) >>
        sp >> op2_k: debug_wrap!(expr) >>
        (Expr::PlusItem(Box::new(Plus {
            op1: op1_k,
            op2: op2_k,
        }))))
    | do_parse!(
        sp >> ident_k: debug_wrap!(ident) >>
        (Expr::VarNameItem(VarName {
            ident: ident_k,
        })))
));

named!(pub source<Source>,
    do_parse!(
        sp >> exprs_k: debug_wrap!(exprs) >>
        (Source {
            exprs: exprs_k,
        }))
);

named!(pub exprs<Vec<Expr>>, separated_list!(debug_wrap!(char!(';')), 
    expr
));

