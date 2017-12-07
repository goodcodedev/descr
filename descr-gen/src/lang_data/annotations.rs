use std::collections::HashMap;
use descr_lang::gen::ast::*;

#[derive(Debug)]
pub struct AnnotList<'a> {
    pub items: HashMap<&'a str, Annot<'a>>
}
#[derive(Debug)]
pub struct Annot<'a> {
    pub ident: &'a str,
    pub args: HashMap<&'a str, AnArgVal<'a>>
}
#[derive(Debug)]
pub enum AnArgVal<'a> {
    Quoted(&'a str),
    Ident(&'a str),
    IntConst(u32)
}

pub fn parse_annots<'a>(annotations: &Vec<Annotation<'a>>) -> AnnotList<'a> {
    let mut l = AnnotList { items: HashMap::new() };
    for annot in annotations {
        let mut a = Annot {
            ident: annot.ident,
            args: HashMap::new()
        };
        if let Some(ref annot_args) = annot.annot_args {
            for annot_arg in &annot_args.annot_arg_list {
                a.args.insert(
                    annot_arg.key,
                    match &annot_arg.annot_arg_val {
                        &AnnotArgVal::QuotedItem(Quoted{string}) => AnArgVal::Quoted(string),
                        &AnnotArgVal::IdentItem(Ident{ident}) => AnArgVal::Ident(ident),
                        &AnnotArgVal::IntConstItem(IntConst{int}) => AnArgVal::IntConst(int)
                    }
                );
            }
        }
        l.items.insert(annot.ident, a);
    }
    println!("Annotlist: {:#?}", l);
    l
}