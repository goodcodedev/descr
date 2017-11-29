use ast::*;
use std::str;
use nom::*;
extern crate descr_common;
use descr_common::parsers::*;

named!(pub source<Source>, do_parse!(
    nodes: ws!(many0!(alt_complete!(
        map!(ast_single, |node| { SourceNode::AstSingle(node) })
        | map!(ast_many, |node| { SourceNode::AstMany(node) })
        | map!(list, |node| { SourceNode::List(node) })
    ))) >>
    (Source {
        nodes: nodes
    })
));

named!(ast_single<AstSingle>, do_parse!(
    ident: ws!(ident) >>
    char!('(') >>
    token_list: many1!(token) >>
    char!(')') >>
    (AstSingle {
        ident: ident,
        token_list : token_list
    })
));

named!(ast_many<AstMany>, do_parse!(
    ident: ws!(ident) >>
    char!('{') >>
    many: separated_list!(ws!(tag!(",")), ast_item) >>
    opt!(multispace) >>
    char!('}') >>
    (AstMany {
        ident: ident,
        ast_items: many
    })
));

named!(ast_item<AstItem>, alt_complete!(
    do_parse!(
        ident: ws!(opt!(ident)) >>
        char!('(') >>
        token_list: many1!(token) >>
        char!(')') >>
        (AstItem::AstDef(AstDef {
            ident: ident,
            token_list: token_list
        }))
    )
    | do_parse!(
        ident: ws!(ident) >>
        (AstItem::AstRef(AstRef {
            ident: ident
        }))
    )
));

named!(list_item<ListItem>, do_parse!(
    ast_item: ast_item >>
    sep: opt!(ident) >>
    (ListItem {
        ast_item: ast_item,
        sep: sep
    })
));

named!(list<List>, do_parse!(
    list_ident: ws!(ident) >>
    char!('[') >>
    char!(']') >>
    sep: ws!(opt!(ident)) >>
    list: alt_complete!(
        do_parse!(
            char!('{') >>
            items: separated_list!(ws!(char!(',')), list_item) >>
            ws!(char!('}')) >>
            (List { 
                ident: list_ident,
                sep: sep,
                items: items
            })
        )
        | do_parse!(
            ast_item: ast_item >>
            (List {
                ident: list_ident,
                sep: sep,
                items: vec![ListItem {
                    ast_item: ast_item,
                    sep: None
                }]
            })
        )
    ) >>
    (list)
));


named!(token<TokenNode>, ws!(alt_complete!(
    do_parse!(
        name: ident >>
        ws!(char!(':')) >>
        key: ident >>
        optional: opt!(char!('?')) >>
        (TokenNode::TokenNamedKey(
            TokenNamedKey {
                name: name,
                key: key,
                optional: match optional {
                    None => false,
                    _ => true
                }
            }
        ))
    )
    | do_parse!(
        ident: ident >>
        optional: opt!(char!('?')) >>
        (TokenNode::TokenKey(
            TokenKey {
                ident: ident,
                optional: match optional {
                    None => false,
                    _ => true
                }
            }
        ))
))));
