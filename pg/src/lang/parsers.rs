use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use std;
use super::ast::*;

named!(pub start<Source>, do_parse!(res: source >> (res)));

named!(pub color<Color>, alt_complete!(
    do_parse!(
        sp >> tag!("red") >>
        (Color::Red        ))
    | do_parse!(
        sp >> tag!("green") >>
        (Color::Green        ))
    | do_parse!(
        sp >> tag!("blue") >>
        (Color::Blue        ))
));

named!(pub source<Source>,
    do_parse!(
        sp >> statements_k: statements >>
        (Source {
            statements: statements_k,
        }))
);

named!(pub statements<Vec<Statement>>, many0!(alt_complete!(
    do_parse!(
        sp >> tag!("say") >>
        sp >> string_k: quoted_str >>
        (Statement::SayItem(Say {
            string: string_k,
        })))
    | do_parse!(
        sp >> tag!("bg") >>
        sp >> color_k: color >>
        (Statement::BgColorItem(BgColor {
            color: color_k,
        })))
)));

