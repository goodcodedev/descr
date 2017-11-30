extern crate descr_common;
use self::descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use super::ast::*;

named!(pub start<Source>, do_parse!(res: source >> (res)));

named!(pub source<Source>,
    do_parse!(
        sp >> items_k: items >>
        (Source {
            items: items_k,
        }))
);

named!(pub items<Vec<SourceItem>>, separated_list!(sp, alt_complete!(
    do_parse!(
        sp >> tag!("say") >>
        sp >> string_k: quoted_str >>
        (SourceItem::SayItem(Say {
            string: string_k,
        })))
    | do_parse!(
        sp >> tag!("hello") >>
        (SourceItem::HelloItem(Hello {
        })))
)));

