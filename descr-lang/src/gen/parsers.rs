extern crate descr_common;
use self::descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use super::ast::*;

named!(pub ast_item<AstItem>, alt_complete!(
    do_parse!(
        sp >> ident_k: opt!(ident) >>
        sp >> char!('(') >>
        sp >> tokens_k: token_list >>
        sp >> char!(')') >>
        (AstItem::AstDefItem(AstDef {
            ident: ident_k,
            tokens: tokens_k,
        })))
    | do_parse!(
        sp >> ident_k: ident >>
        (AstItem::AstRefItem(AstRef {
            ident: ident_k,
        })))
));

named!(pub ast_many<AstMany>,
    do_parse!(
        sp >> ident_k: ident >>
        sp >> char!('{') >>
        sp >> items_k: ast_items >>
        sp >> char!('}') >>
        (AstMany {
            ident: ident_k,
            items: items_k,
        }))
);

named!(pub ast_single<AstSingle>,
    do_parse!(
        sp >> ident_k: ident >>
        sp >> char!('(') >>
        sp >> tokens_k: token_list >>
        sp >> char!(')') >>
        (AstSingle {
            ident: ident_k,
            tokens: tokens_k,
        }))
);

named!(pub list<List>, alt_complete!(
    do_parse!(
        sp >> ident_k: ident >>
        sp >> sep_k: ident >>
        sp >> reference_k: ident >>
        (List::ListSingleItem(ListSingle {
            ident: ident_k,
            sep: sep_k,
            reference: reference_k,
        })))
    | do_parse!(
        sp >> ident_k: ident >>
        sp >> sep_k: opt!(ident) >>
        sp >> char!('{') >>
        sp >> items_k: list_items >>
        sp >> char!('}') >>
        (List::ListManyItem(ListMany {
            ident: ident_k,
            sep: sep_k,
            items: items_k,
        })))
));

named!(pub list_item<ListItem>,
    do_parse!(
        sp >> ident_k: ident >>
        sp >> ast_item_k: ast_item >>
        sp >> sep_k: opt!(ident) >>
        (ListItem {
            ident: ident_k,
            ast_item: ast_item_k,
            sep: sep_k,
        }))
);

named!(pub source<Source>,
    do_parse!(
        sp >> items_k: source_items >>
        (Source {
            items: items_k,
        }))
);

named!(pub source_item<SourceItem>, alt_complete!(
    map!(ast_single, |node| { SourceItem::AstSingleItem(node) })
    | map!(ast_many, |node| { SourceItem::AstManyItem(node) })
    | map!(list, |node| { SourceItem::ListItem(node) })
));

named!(pub token<Token>, alt_complete!(
    do_parse!(
        sp >> ident_k: ident >>
        sp >> optional_k: opt!(char!('?')) >>
        (Token::TokenKeyItem(TokenKey {
            ident: ident_k,
            optional: optional_k.is_some(),
        })))
    | do_parse!(
        sp >> name_k: ident >>
        sp >> char!(':') >>
        sp >> key_k: ident >>
        sp >> optional_k: opt!(char!('?')) >>
        (Token::TokenNamedKeyItem(TokenNamedKey {
            name: name_k,
            key: key_k,
            optional: optional_k.is_some(),
        })))
));

named!(pub ast_items<Vec<AstItem>>, separated_list!(char!(','), 
    ast_item
));

named!(pub list_items<Vec<ListItem>>, separated_list!(char!(','), 
    list_item
));

named!(pub source_items<Vec<SourceItem>>, separated_list!(multispace, 
    source_item
));

named!(pub token_list<Vec<Token>>, separated_list!(multispace, 
    token
));

