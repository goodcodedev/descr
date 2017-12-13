#[allow(unused_imports)]
use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
#[allow(unused_imports)]
use std;
use super::ast::*;

named!(pub start<Test>, do_parse!(res: test >> (res)));

named!(pub test<Test>,
    do_parse!(
        sp >> key1_k: tag!("First") >>
        sp >> key2_k: tag!("Second") >>
        (Test {
            key1: true,
            key2: true,
        }))
);

