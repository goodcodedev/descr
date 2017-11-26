named!(Source<Source>, alt_complete!(
AstSingle
| AstMany
| List
| ));
named!(AstItem<AstItem>, alt_complete!(
    ident >>
    char!('(') >>
    tokenList >>
    char!(')') >>

|     ident >>

| ));
named!(AstSingle<AstSingle>,
    ident >>
    char!('(') >>
    tokenList >>
    char!(')') >>
);
named!(AstMany<AstMany>,
    ident >>
    char!('{') >>
    astItems >>
    char!('}') >>
);
named!(Token<Token>, alt_complete!(
    ident >>
    char!('?') >>

|     ident >>
    char!(':') >>
    ident >>
    char!('?') >>

| ));
