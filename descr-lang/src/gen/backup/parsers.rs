use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use std;
use super::ast::*;

named!(pub start<Source>, do_parse!(res: source >> (res)));

named!(pub annot_arg_val<AnnotArgVal>, alt_complete!(
    do_parse!(
        sp >> string_k: debug_wrap!(quoted_str) >>
        (AnnotArgVal::QuotedItem(Quoted {
            string: string_k,
        })))
    | do_parse!(
        sp >> ident_k: debug_wrap!(ident) >>
        (AnnotArgVal::IdentItem(Ident {
            ident: ident_k,
        })))
    | do_parse!(
        sp >> int_k: debug_wrap!(parse_int) >>
        (AnnotArgVal::IntConstItem(IntConst {
            int: int_k,
        })))
));

named!(pub annot_args<AnnotArgs>,
    do_parse!(
        sp >> debug_wrap!(char!('(')) >>
        sp >> annot_arg_list_k: debug_wrap!(annot_arg_list) >>
        sp >> debug_wrap!(char!(')')) >>
        (AnnotArgs {
            annot_arg_list: annot_arg_list_k,
        }))
);

named!(pub annotation<Annotation>,
    do_parse!(
        sp >> debug_wrap!(tag!("@")) >>
        sp >> ident_k: debug_wrap!(ident) >>
        annot_args_k: opt!(do_parse!(sp >> res: debug_wrap!(annot_args) >> (res))) >>
        (Annotation {
            ident: ident_k,
            annot_args: annot_args_k,
        }))
);

named!(pub ast_item<AstItem>, alt_complete!(
    do_parse!(
        sp >> annots_k: debug_wrap!(annots) >>
        sp >> tokens_k: debug_wrap!(token_list) >>
        sp >> debug_wrap!(tag!("=>")) >>
        ident_k: opt!(do_parse!(sp >> res: debug_wrap!(ident) >> (res))) >>
        (AstItem::AstDefItem(AstDef {
            annots: annots_k,
            tokens: tokens_k,
            ident: ident_k,
        })))
    | do_parse!(
        sp >> annots_k: debug_wrap!(annots) >>
        sp >> debug_wrap!(char!('(')) >>
        sp >> tokens_k: debug_wrap!(token_list) >>
        sp >> debug_wrap!(char!(')')) >>
        sp >> debug_wrap!(tag!("=>")) >>
        ident_k: opt!(do_parse!(sp >> res: debug_wrap!(ident) >> (res))) >>
        (AstItem::AstDefItem(AstDef {
            annots: annots_k,
            tokens: tokens_k,
            ident: ident_k,
        })))
    | do_parse!(
        sp >> annots_k: debug_wrap!(annots) >>
        ident_k: opt!(do_parse!(sp >> res: debug_wrap!(ident) >> (res))) >>
        sp >> debug_wrap!(char!('(')) >>
        sp >> tokens_k: debug_wrap!(token_list) >>
        sp >> debug_wrap!(char!(')')) >>
        (AstItem::AstDefItem(AstDef {
            annots: annots_k,
            ident: ident_k,
            tokens: tokens_k,
        })))
    | do_parse!(
        sp >> ident_k: debug_wrap!(ident) >>
        (AstItem::AstRefItem(AstRef {
            ident: ident_k,
        })))
));

named!(pub ast_many<AstMany>,
    do_parse!(
        sp >> annots_k: debug_wrap!(annots) >>
        sp >> ident_k: debug_wrap!(ident) >>
        sp >> debug_wrap!(char!('{')) >>
        sp >> items_k: debug_wrap!(ast_items) >>
        sp >> debug_wrap!(char!('}')) >>
        (AstMany {
            annots: annots_k,
            ident: ident_k,
            items: items_k,
        }))
);

named!(pub ast_single<AstSingle>,
    do_parse!(
        sp >> annots_k: debug_wrap!(annots) >>
        sp >> ident_k: debug_wrap!(ident) >>
        sp >> debug_wrap!(char!('(')) >>
        sp >> tokens_k: debug_wrap!(token_list) >>
        sp >> debug_wrap!(char!(')')) >>
        (AstSingle {
            annots: annots_k,
            ident: ident_k,
            tokens: tokens_k,
        }))
);

named!(pub comment<Comment>,
    do_parse!(
        sp >> debug_wrap!(tag!("(*")) >>
        comment_k: until_done_result!(debug_wrap!(tag!("*)"))) >>
        sp >> debug_wrap!(tag!("*)")) >>
        (Comment {
            comment: std::str::from_utf8(comment_k).unwrap(),
        }))
);

named!(pub list<List>, alt_complete!(
    do_parse!(
        sp >> annots_k: debug_wrap!(annots) >>
        sp >> ident_k: debug_wrap!(ident) >>
        sp >> debug_wrap!(char!('[')) >>
        sp >> debug_wrap!(char!(']')) >>
        sp >> sep_k: debug_wrap!(ident) >>
        sp >> reference_k: debug_wrap!(ident) >>
        (List::ListSingleItem(ListSingle {
            annots: annots_k,
            ident: ident_k,
            sep: sep_k,
            reference: reference_k,
        })))
    | do_parse!(
        sp >> annots_k: debug_wrap!(annots) >>
        sp >> ident_k: debug_wrap!(ident) >>
        sp >> debug_wrap!(char!(':')) >>
        sp >> ast_type_k: debug_wrap!(ident) >>
        sp >> debug_wrap!(char!('[')) >>
        sp >> debug_wrap!(char!(']')) >>
        sep_k: opt!(do_parse!(sp >> res: debug_wrap!(ident) >> (res))) >>
        sp >> debug_wrap!(char!('{')) >>
        sp >> items_k: debug_wrap!(list_items) >>
        sp >> debug_wrap!(char!('}')) >>
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
        sp >> ast_item_k: debug_wrap!(ast_item) >>
        sep_k: opt!(do_parse!(sp >> res: debug_wrap!(ident) >> (res))) >>
        (ListItem {
            ast_item: ast_item_k,
            sep: sep_k,
        }))
);

named!(pub source<Source>,
    do_parse!(
        sp >> items_k: debug_wrap!(source_items) >>
        (Source {
            items: items_k,
        }))
);

named!(pub token_type<TokenType>, alt_complete!(
    do_parse!(
        sp >> ident_k: debug_wrap!(ident) >>
        sp >> debug_wrap!(char!('(')) >>
        sp >> fn_args_k: debug_wrap!(fn_args) >>
        sp >> debug_wrap!(char!(')')) >>
        (TokenType::FuncTokenItem(FuncToken {
            ident: ident_k,
            fn_args: fn_args_k,
        })))
    | do_parse!(
        sp >> key_k: debug_wrap!(ident) >>
        (TokenType::KeyTokenItem(KeyToken {
            key: key_k,
        })))
    | do_parse!(
        sp >> string_k: debug_wrap!(quoted_str) >>
        (TokenType::QuotedItem(Quoted {
            string: string_k,
        })))
));

named!(pub annot_arg_list<Vec<AnnotArg>>, separated_list!(debug_wrap!(char!(',')), 
    do_parse!(
        sp >> key_k: debug_wrap!(ident) >>
        sp >> debug_wrap!(char!('=')) >>
        sp >> annot_arg_val_k: debug_wrap!(annot_arg_val) >>
        (AnnotArg {
            key: key_k,
            annot_arg_val: annot_arg_val_k,
        }))
));

named!(pub annots<Vec<Annotation>>, many0!(
    annotation
));

named!(pub ast_items<Vec<AstItem>>, separated_list!(debug_wrap!(char!(',')), 
    ast_item
));

named!(pub fn_args<Vec<FuncArg>>, separated_list!(debug_wrap!(char!(',')), 
    do_parse!(
        sp >> string_k: debug_wrap!(quoted_str) >>
        (FuncArg::QuotedItem(Quoted {
            string: string_k,
        })))
));

named!(pub list_items<Vec<ListItem>>, separated_list!(debug_wrap!(char!(',')), 
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
        sp >> annots_k: debug_wrap!(annots) >>
        sp >> name_k: debug_wrap!(ident) >>
        sp >> debug_wrap!(char!(':')) >>
        not_k: opt!(do_parse!(sp >> res: debug_wrap!(char!('!')) >> (res))) >>
        sp >> token_type_k: debug_wrap!(token_type) >>
        optional_k: opt!(do_parse!(sp >> res: debug_wrap!(char!('?')) >> (res))) >>
        (Token::NamedTokenItem(NamedToken {
            annots: annots_k,
            name: name_k,
            not: not_k.is_some(),
            token_type: token_type_k,
            optional: optional_k.is_some(),
        })))
    | do_parse!(
        sp >> annots_k: debug_wrap!(annots) >>
        not_k: opt!(do_parse!(sp >> res: debug_wrap!(char!('!')) >> (res))) >>
        sp >> token_type_k: debug_wrap!(token_type) >>
        optional_k: opt!(do_parse!(sp >> res: debug_wrap!(char!('?')) >> (res))) >>
        (Token::SimpleTokenItem(SimpleToken {
            annots: annots_k,
            not: not_k.is_some(),
            token_type: token_type_k,
            optional: optional_k.is_some(),
        })))
    | do_parse!(
        sp >> annots_k: debug_wrap!(annots) >>
        not_k: opt!(do_parse!(sp >> res: debug_wrap!(char!('!')) >> (res))) >>
        sp >> debug_wrap!(char!('{')) >>
        sp >> token_list_k: debug_wrap!(token_list) >>
        sp >> debug_wrap!(char!('}')) >>
        optional_k: opt!(do_parse!(sp >> res: debug_wrap!(char!('?')) >> (res))) >>
        (Token::TokenGroupItem(TokenGroup {
            annots: annots_k,
            not: not_k.is_some(),
            token_list: token_list_k,
            optional: optional_k.is_some(),
        })))
)));

