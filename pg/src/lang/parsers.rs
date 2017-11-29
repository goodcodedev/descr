extern crate descr_common;
use self::descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use super::ast::*;

named!(pub start<First>, do_parse!(res: first >> (res)));

named!(pub first<First>,
    do_parse!(
        sp >> char!('(') >>
        sp >> second_k: second >>
        sp >> char!(')') >>
        (First {
            second: second_k,
        }))
);

named!(pub second<Second>,
    do_parse!(
        sp >> ident_k: ident >>
        (Second {
            ident: ident_k,
        }))
);

