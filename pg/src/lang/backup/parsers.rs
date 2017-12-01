use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use std;
use super::ast::*;

named!(pub start<Source>, do_parse!(res: source >> (res)));

named!(pub source<Source>,
    do_parse!(
        sp >> source_items_k: source_items >>
        (Source {
            source_items: source_items_k,
        }))
);

named!(pub source_items<SourceItems>, alt_complete!(
    do_parse!(
        sp >> tag!("test") >>
        sp >> num_k: parse_int >>
        (SourceItems::RandomItem(Random {
            num: num_k,
        })))
    | do_parse!(
        sp >> tag!("(*") >>
        comment_k: until_done_result!(tag!("*)")) >>
        sp >> tag!("*)") >>
        (SourceItems::CommentItem(Comment {
            comment: std::str::from_utf8(comment_k).unwrap(),
        })))
));

