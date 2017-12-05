use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use std;
use super::ast::*;

named!(pub start<Expr>, do_parse!(res: expr >> (res)));

named!(pub expr<Expr>, alt_complete!(
    do_parse!(
        sp >> ident_k: ident >>
        (Expr::VarNameItem(VarName {
            ident: ident_k,
        })))
    | do_parse!(
        sp >> op1_k: expr >>
        sp >> tag!("+") >>
        sp >> op2_k: expr >>
        (Expr::PlusItem(Box::new(Plus {
            op1: op1_k,
            op2: op2_k,
        }))))
));

