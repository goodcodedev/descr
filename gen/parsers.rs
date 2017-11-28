named!(ast_item<AstItem>, alt_complete!(
    do_parse!(
        sp >> ident_k: opt!(ident) >>
        sp >> char!('(') >>
        sp >> tokens_k: tokenList >>
        sp >> char!(')') >>
        (AstItem(AstDefItem {
            ident: ident_k,
            tokens: tokens_k,
        })))
    | do_parse!(
        sp >> ident_k: ident >>
        (AstItem(AstRefItem {
            ident: ident_k,
        })))
));

named!(ast_many<AstMany>,
    do_parse!(
        sp >> ident_k: ident >>
        sp >> char!('{') >>
        sp >> items_k: astItems >>
        sp >> char!('}') >>
        (AstMany {
            ident: ident_k,
            items: items_k,
        }))
);

named!(ast_single<AstSingle>,
    do_parse!(
        sp >> ident_k: ident >>
        sp >> char!('(') >>
        sp >> tokens_k: tokenList >>
        sp >> char!(')') >>
        (AstSingle {
            ident: ident_k,
            tokens: tokens_k,
        }))
);

named!(list<List>, alt_complete!(
    do_parse!(
        sp >> ident_k: ident >>
        sp >> sep_k: ident >>
        sp >> reference_k: ident >>
        (List(ListSingleItem {
            ident: ident_k,
            sep: sep_k,
            reference: reference_k,
        })))
    | do_parse!(
        sp >> ident_k: ident >>
        sp >> sep_k: opt!(ident) >>
        sp >> char!('{') >>
        sp >> items_k: listItems >>
        sp >> char!('}') >>
        (List(ListManyItem {
            ident: ident_k,
            sep: sep_k,
            items: items_k,
        })))
));

named!(list_item<ListItem>,
    do_parse!(
        sp >> ident_k: ident >>
        sp >> ast_item_k: AstItem >>
        sp >> sep_k: opt!(ident) >>
        (ListItem {
            ident: ident_k,
            ast_item: AstItem_k,
            sep: sep_k,
        }))
);

named!(source<Source>, alt_complete!(
    ast_single
    | ast_many
    | list
));

named!(token<Token>, alt_complete!(
    do_parse!(
        sp >> ident_k: ident >>
        sp >> optional_k: opt!(char!('?')) >>
        (Token(TokenKeyItem {
            ident: ident_k,
            optional: optional_k.is_some(),
        })))
    | do_parse!(
        sp >> name_k: ident >>
        sp >> char!(':') >>
        sp >> key_k: ident >>
        sp >> optional_k: opt!(char!('?')) >>
        (Token(TokenNamedKeyItem {
            name: name_k,
            key: key_k,
            optional: optional_k.is_some(),
        })))
));

named!(astItems, many0!(
    ast_item
));

named!(listItems, many0!(
    list_item
));

named!(tokenList, many0!(
    token
));

