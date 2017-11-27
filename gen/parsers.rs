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

