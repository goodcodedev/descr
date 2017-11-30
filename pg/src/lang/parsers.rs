use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use super::ast::*;

named!(pub start<Source>, do_parse!(res: source >> (res)));

named!(pub comment<Comment>,
    do_parse!(
        sp >> tag!("(*") >>
        until_done_result!(tag!("*)")) >>
        sp >> tag!("*)") >>
        (Comment {
        }))
);

named!(pub source<Source>,
    do_parse!(
        sp >> items_k: source_items >>
        (Source {
            items: items_k,
        }))
);

named!(pub source_items<Vec<SourceItem>>, separated_list!(sp, alt_complete!(
    do_parse!(
        sp >> tag!("test") >>
        sp >> string_k: quoted_str >>
        (SourceItem::RandomItem(Random {
            string: string_k,
        })))
    | map!(comment, |node| { SourceItem::CommentItem(node) })
)));

