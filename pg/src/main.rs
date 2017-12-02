#[macro_use]
extern crate descr_common;
mod lang;
extern crate nom;

fn main() {
    let buf = descr_common::util::load_file("../pg-example.pg");
    let res = lang::parsers::start(&buf[..]);
    println!("\n= Result ===========================");
    println!("{:#?}", res);
    let mut v = Interpr {};
    match res {
        nom::IResult::Done(_, ref source) => v.visit_source(source),
        _ => {}
    }
}

fn set_bg(r: u32, g: u32, b: u32) {
    println!("Setting bg: {}, {}, {}", r, g, b);
}

use lang::visitor::Visitor;
use lang::ast::*;

struct Interpr;
impl<'a> Visitor<'a> for Interpr {

    fn visit_say(&mut self, node: &'a Say) {
        println!("Saying: {}", node.string);
    }

    fn visit_bg_color(&mut self, node: &'a BgColor) {
        use Color::*;
        match &node.color {
            &Red => set_bg(255, 180, 180),
            &Green => set_bg(180, 255, 180),
            &Blue => set_bg(180, 180, 255)
        };
    }
}