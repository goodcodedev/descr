named!(AstItem<AstItem>, alt_complete!(
    do_parse!(
        sp >> ident_k: opt!(ident) >>
        sp >> char!('(') >>
        sp >> tokenList_k: tokenList >>
        sp >> char!(')') >>
        (AstItem(AstDefItem {
            ident: ident_k,
            tokenList: tokenList_k,
        })))
    | do_parse!(
        sp >> ident_k: ident >>
        (AstItem(AstRefItem {
            ident: ident_k,
        })))
));

named!(AstMany<AstMany>,
    do_parse!(
        sp >> ident_k: ident >>
        sp >> char!('{') >>
        sp >> astItems_k: astItems >>
        sp >> char!('}') >>
        (AstMany {
            ident: ident_k,
            astItems: astItems_k,
        }))
);

named!(AstSingle<AstSingle>,
    do_parse!(
        sp >> ident_k: ident >>
        sp >> char!('(') >>
        sp >> tokenList_k: tokenList >>
        sp >> char!(')') >>
        (AstSingle {
            ident: ident_k,
            tokenList: tokenList_k,
        }))
);

named!(List<List>, alt_complete!(
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
        sp >> ListItem_k: ListItem >>
        sp >> char!('}') >>
        (List(ListManyItem {
            ident: ident_k,
            sep: sep_k,
            ListItem: ListItem_k,
        })))
));

named!(ListItem<ListItem>,
    do_parse!(
        sp >> ident_k: ident >>
        sp >> char!('(') >>
        sp >> tokenList_k: tokenList >>
        sp >> char!(')') >>
        sp >> sep_k: opt!(ident) >>
        (ListItem {
            ident: ident_k,
            tokenList: tokenList_k,
            sep: sep_k,
        }))
);

named!(Source<Source>, alt_complete!(
    AstSingle
    | AstMany
    | List
));

named!(Token<Token>, alt_complete!(
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
    AstItem
));

named!(tokenList, many0!(
    Token
));

