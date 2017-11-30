extern crate descr_common;
use self::descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use super::ast::*;

named!(pub start<Source>, do_parse!(res: source >> (res)));

named!(pub something<Something>, alt_complete!(
    do_parse!(
        sp >> tag!("tag") >>
        sp >> string_k: quoted_str >>
        (Something::StrItemItem(StrItem {
            string: string_k,
        })))
    | do_parse!(
        sp >> tag!("tag2") >>
        sp >> string_k: quoted_str >>
        (Something::Str2ItemItem(Str2Item {
            string: string_k,
        })))
));

named!(pub source<Source>,
    do_parse!(
        sp >> some_list_k: some_list >>
        (Source {
            some_list: some_list_k,
        }))
);

named!(pub some_list<Vec<Something>>, separated_list!(char!(','), 
    something
));

