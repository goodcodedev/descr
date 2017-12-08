use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use std;
use super::ast::*;

named!(pub start<Source>, do_parse!(res: source >> (res)));

named!(pub generic<Generic>,
    do_parse!(
        sp >> tag!("<") >>
        sp >> generic_items_k: generic_items >>
        sp >> tag!(">") >>
        (Generic {
            generic_items: generic_items_k,
        }))
);

named!(pub generic_item<GenericItem>, alt_complete!(
    map!(life_time, |node| { GenericItem::LifeTimeItem(node) })
    | do_parse!(
        sp >> ident_k: ident >>
        (GenericItem::GenTypeItem(GenType {
            ident: ident_k,
        })))
));

named!(pub life_time<LifeTime>,
    do_parse!(
        sp >> tag!("'") >>
        sp >> ident_k: ident >>
        (LifeTime {
            ident: ident_k,
        }))
);

named!(pub rs_enum<RsEnum>,
    do_parse!(
        public_k: opt!(do_parse!(sp >> res: tag!("pub") >> (res))) >>
        sp >> tag!("enum") >>
        sp >> ident_k: ident >>
        sp >> char!('{') >>
        sp >> enum_items_k: enum_items >>
        sp >> char!('}') >>
        (RsEnum {
            public: public_k.is_some(),
            ident: ident_k,
            enum_items: enum_items_k,
        }))
);

named!(pub rs_struct<RsStruct>,
    do_parse!(
        public_k: opt!(do_parse!(sp >> res: tag!("pub") >> (res))) >>
        sp >> tag!("struct") >>
        sp >> ident_k: ident >>
        generic_k: opt!(do_parse!(sp >> res: generic >> (res))) >>
        sp >> char!('{') >>
        sp >> struct_members_k: struct_members >>
        sp >> char!('}') >>
        (RsStruct {
            public: public_k.is_some(),
            ident: ident_k,
            generic: generic_k,
            struct_members: struct_members_k,
        }))
);

named!(pub rs_trait<RsTrait>,
    do_parse!(
        public_k: opt!(do_parse!(sp >> res: tag!("pub") >> (res))) >>
        sp >> tag!("trait") >>
        sp >> ident_k: ident >>
        sp >> char!('{') >>
        sp >> char!('}') >>
        (RsTrait {
            public: public_k.is_some(),
            ident: ident_k,
        }))
);

named!(pub source<Source>,
    do_parse!(
        sp >> source_items_k: source_items >>
        (Source {
            source_items: source_items_k,
        }))
);

named!(pub tpe<Tpe>, alt_complete!(
    do_parse!(
        sp >> tag!("u32") >>
        (Tpe::RsU32        ))
    | do_parse!(
        sp >> tag!("i32") >>
        (Tpe::RsI32        ))
    | do_parse!(
        sp >> tag!("String") >>
        (Tpe::RsString        ))
    | do_parse!(
        sp >> tag!("str") >>
        (Tpe::RsStr        ))
    | do_parse!(
        sp >> tag!("bool") >>
        (Tpe::Bool        ))
));

named!(pub tpe_spes<TpeSpes>,
    do_parse!(
        sp >> tpe_k: tpe >>
        generic_item_k: opt!(do_parse!(sp >> res: generic_item >> (res))) >>
        (TpeSpes {
            tpe: tpe_k,
            generic_item: generic_item_k,
        }))
);

named!(pub enum_items<Vec<EnumItem>>, separated_list!(char!(','), 
    do_parse!(
        sp >> ident_k: ident >>
        (EnumItem {
            ident: ident_k,
        }))
));

named!(pub generic_items<Vec<GenericItem>>, separated_list!(char!(','), 
    generic_item
));

named!(pub source_items<Vec<SourceItem>>, many0!(alt_complete!(
    map!(rs_struct, |node| { SourceItem::RsStructItem(node) })
    | map!(rs_enum, |node| { SourceItem::RsEnumItem(node) })
    | map!(rs_trait, |node| { SourceItem::RsTraitItem(node) })
)));

named!(pub struct_members<Vec<StructMember>>, separated_list!(char!(','), 
    do_parse!(
        public_k: opt!(do_parse!(sp >> res: tag!("pub") >> (res))) >>
        sp >> ident_k: ident >>
        sp >> char!(':') >>
        sp >> tpe_spes_k: tpe_spes >>
        (StructMember {
            public: public_k.is_some(),
            ident: ident_k,
            tpe_spes: tpe_spes_k,
        }))
));

