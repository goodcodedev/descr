#[allow(unused_imports)]
use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
#[allow(unused_imports)]
use std;
use super::ast::*;

named!(pub start<Source>, do_parse!(res: source >> (res)));

named!(pub annot_arg_val<AnnotArgVal>, alt_complete!(
    do_parse!(
        sp >> string_k: quoted_str >>
        (AnnotArgVal::QuotedItem(Quoted {
            string: string_k,
        })))
    | do_parse!(
        sp >> ident_k: ident >>
        (AnnotArgVal::IdentItem(Ident {
            ident: ident_k,
        })))
    | do_parse!(
        sp >> int_k: parse_int >>
        (AnnotArgVal::IntConstItem(IntConst {
            int: int_k,
        })))
));

named!(pub annot_args<AnnotArgs>,
    do_parse!(
        sp >> char!('(') >>
        sp >> annot_arg_list_k: annot_arg_list >>
        sp >> char!(')') >>
        (AnnotArgs {
            annot_arg_list: annot_arg_list_k,
        }))
);

named!(pub annotation<Annotation>,
    do_parse!(
        sp >> tag!("@") >>
        sp >> ident_k: ident >>
        annot_args_k: opt!(do_parse!(sp >> res: annot_args >> (res))) >>
        (Annotation {
            ident: ident_k,
            annot_args: annot_args_k,
        }))
);

named!(pub ast_item<AstItem>, alt_complete!(
    do_parse!(
        sp >> annots_k: annots >>
        sp >> tokens_k: token_list >>
        sp >> tag!("=>") >>
        ident_k: opt!(do_parse!(sp >> res: ident >> (res))) >>
        (AstItem::AstDefItem(AstDef {
            annots: annots_k,
            tokens: tokens_k,
            ident: ident_k,
        })))
    | do_parse!(
        sp >> annots_k: annots >>
        sp >> char!('(') >>
        sp >> tokens_k: token_list >>
        sp >> char!(')') >>
        sp >> tag!("=>") >>
        ident_k: opt!(do_parse!(sp >> res: ident >> (res))) >>
        (AstItem::AstDefItem(AstDef {
            annots: annots_k,
            tokens: tokens_k,
            ident: ident_k,
        })))
    | do_parse!(
        sp >> annots_k: annots >>
        ident_k: opt!(do_parse!(sp >> res: ident >> (res))) >>
        sp >> char!('(') >>
        sp >> tokens_k: token_list >>
        sp >> char!(')') >>
        (AstItem::AstDefItem(AstDef {
            annots: annots_k,
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
        sp >> annots_k: annots >>
        sp >> ident_k: ident >>
        sp >> char!('{') >>
        sp >> items_k: ast_items >>
        sp >> char!('}') >>
        (AstMany {
            annots: annots_k,
            ident: ident_k,
            items: items_k,
        }))
);

named!(pub ast_single<AstSingle>,
    do_parse!(
        sp >> annots_k: annots >>
        sp >> ident_k: ident >>
        sp >> char!('(') >>
        sp >> tokens_k: token_list >>
        sp >> char!(')') >>
        (AstSingle {
            annots: annots_k,
            ident: ident_k,
            tokens: tokens_k,
        }))
);

named!(pub comment<Comment>,
    do_parse!(
        sp >> tag!("(*") >>
        comment_k: until_done_result!(tag!("*)")) >>
        sp >> tag!("*)") >>
        (Comment {
            comment: std::str::from_utf8(comment_k).unwrap(),
        }))
);

named!(pub list<List>, alt_complete!(
    do_parse!(
        sp >> annots_k: annots >>
        sp >> ident_k: ident >>
        sp >> char!('[') >>
        sp >> char!(']') >>
        sp >> sep_k: ident >>
        sp >> reference_k: ident >>
        (List::ListSingleItem(ListSingle {
            annots: annots_k,
            ident: ident_k,
            sep: sep_k,
            reference: reference_k,
        })))
    | do_parse!(
        sp >> annots_k: annots >>
        sp >> ident_k: ident >>
        sp >> char!(':') >>
        sp >> ast_type_k: ident >>
        sp >> char!('[') >>
        sp >> char!(']') >>
        sep_k: opt!(do_parse!(sp >> res: ident >> (res))) >>
        sp >> char!('{') >>
        sp >> items_k: list_items >>
        sp >> char!('}') >>
        (List::ListManyItem(ListMany {
            annots: annots_k,
            ident: ident_k,
            ast_type: ast_type_k,
            sep: sep_k,
            items: items_k,
        })))
));

named!(pub list_item<ListItem>,
    do_parse!(
        sp >> ast_item_k: ast_item >>
        sep_k: opt!(do_parse!(sp >> res: ident >> (res))) >>
        (ListItem {
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

named!(pub token_type<TokenType>, alt_complete!(
    do_parse!(
        sp >> ident_k: ident >>
        sp >> char!('(') >>
        sp >> fn_args_k: fn_args >>
        sp >> char!(')') >>
        (TokenType::FuncTokenItem(FuncToken {
            ident: ident_k,
            fn_args: fn_args_k,
        })))
    | do_parse!(
        sp >> key_k: ident >>
        (TokenType::KeyTokenItem(KeyToken {
            key: key_k,
        })))
    | do_parse!(
        sp >> string_k: quoted_str >>
        (TokenType::QuotedItem(Quoted {
            string: string_k,
        })))
));

named!(pub annot_arg_list<Vec<AnnotArg>>, separated_list!(char!(','), 
    do_parse!(
        sp >> key_k: ident >>
        sp >> char!('=') >>
        sp >> annot_arg_val_k: annot_arg_val >>
        (AnnotArg {
            key: key_k,
            annot_arg_val: annot_arg_val_k,
        }))
));

named!(pub annots<Vec<Annotation>>, many0!(
    annotation
));

named!(pub ast_items<Vec<AstItem>>, separated_list!(char!(','), 
    ast_item
));

named!(pub fn_args<Vec<FuncArg>>, separated_list!(char!(','), 
    do_parse!(
        sp >> string_k: quoted_str >>
        (FuncArg::QuotedItem(Quoted {
            string: string_k,
        })))
));

named!(pub list_items<Vec<ListItem>>, separated_list!(char!(','), 
    list_item
));

named!(pub source_items<Vec<SourceItem>>, many0!(alt_complete!(
    map!(ast_single, |node| { SourceItem::AstSingleItem(node) })
    | map!(ast_many, |node| { SourceItem::AstManyItem(node) })
    | map!(list, |node| { SourceItem::ListItem(node) })
    | map!(comment, |node| { SourceItem::CommentItem(node) })
)));

named!(pub token_list<Vec<Token>>, many0!(alt_complete!(
    do_parse!(
        sp >> annots_k: annots >>
        sp >> name_k: ident >>
        sp >> char!(':') >>
        not_k: opt!(do_parse!(sp >> res: char!('!') >> (res))) >>
        sp >> token_type_k: token_type >>
        optional_k: opt!(do_parse!(sp >> res: char!('?') >> (res))) >>
        (Token::NamedTokenItem(NamedToken {
            annots: annots_k,
            name: name_k,
            not: not_k.is_some(),
            token_type: token_type_k,
            optional: optional_k.is_some(),
        })))
    | do_parse!(
        sp >> annots_k: annots >>
        not_k: opt!(do_parse!(sp >> res: char!('!') >> (res))) >>
        sp >> token_type_k: token_type >>
        optional_k: opt!(do_parse!(sp >> res: char!('?') >> (res))) >>
        (Token::SimpleTokenItem(SimpleToken {
            annots: annots_k,
            not: not_k.is_some(),
            token_type: token_type_k,
            optional: optional_k.is_some(),
        })))
    | do_parse!(
        sp >> annots_k: annots >>
        not_k: opt!(do_parse!(sp >> res: char!('!') >> (res))) >>
        sp >> char!('(') >>
        sp >> token_list_k: token_list >>
        sp >> char!(')') >>
        optional_k: opt!(do_parse!(sp >> res: char!('?') >> (res))) >>
        (Token::TokenGroupItem(TokenGroup {
            annots: annots_k,
            not: not_k.is_some(),
            token_list: token_list_k,
            optional: optional_k.is_some(),
        })))
)));

