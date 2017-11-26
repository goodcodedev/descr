use ast::*;
use std::str;
use nom::*;

// Quoted with " or single '
// then escapes quote, \ and n with \
named!(quoted_str<&str>, map_res!(alt_complete!(
    delimited!(
        tag!("\""),
        escaped!(
            is_not!("\\\""),
            '\\',
            one_of!("\"\\n")
        ), 
        tag!("\"")
    )
    | delimited!(
        tag!("'"), 
        escaped!(is_not!("\\'"), '\\', one_of!("\\n'")), 
        tag!("'")
    )
), str::from_utf8));

// Identifier starting with alpha
// or _, then alphanumeric + _
fn ident(input: &[u8]) -> IResult<&[u8], &str> {
    if input.len() == 0 {
        return IResult::Incomplete(Needed::Size(1));
    } else {
        let first = input[0];
        if first.is_alpha() || first == '_' as u8 {
            for (i, val) in input[1..].iter().enumerate() {
                if !(val.is_alphanum() || val.as_char() == '_') {
                    return IResult::Done(&input[i+1..], str::from_utf8(&input[..i+1]).unwrap());
                }
            }
            return IResult::Done(&input[input.len()..], str::from_utf8(&input[..input.len()]).unwrap());
        } else {
            return IResult::Error(error_code!(ErrorKind::Custom(42)));
        }
    }
}

// Some special chars that could possibly
// be parsed without quotes
fn special_chars(input: &[u8]) -> IResult<&[u8], &str> {
    if input.len() == 0 {
        return IResult::Incomplete(Needed::Size(1));
    } else {
        let mut i = 0;
        while i <  input.len() {
            if !(input[i] == 0x21 // !, then skip quote (")
                || (input[i] >= 0x23 && input[i] <= 0x26) // # - &, then skip single quote
                || input[i] == 0x28 // (, then skip )
                || (input[i] >= 0x2A && input[i] <= 0x2F) // * - /
                || (input[i] >= 0x3A && input[i] <= 0x40) // : - @
                || (input[i] >= 0x5B && input[i] <= 0x60) // [ - `
                || (input[i] >= 0x7B && input[i] <= 0x7E)) // { - ~
            {
                break;
            }
            i += 1;
        }
        if i > 0 {
            return IResult::Done(&input[i..], str::from_utf8(&input[0..i]).unwrap());
        } else {
            return IResult::Error(error_code!(ErrorKind::Custom(41)));
        }
    }
}

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
