use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use std;
use super::ast::*;

named!(pub start<Source>, do_parse!(res: source >> (res)));

named!(pub source<Source>,
    do_parse!(
        sp >> items_k: source_items >>
        (Source {
            items: items_k,
        }))
);

named!(pub source_items<Vec<SourceItem>>, many0!(
    do_parse!(
        sp >> ident_k: ident >>
        sp >> tag!("val1") >>
        sp >> tag!("val2") >>
        (SourceItem::TestItemItem(TestItem {
            ident: ident_k,
        })))
));

