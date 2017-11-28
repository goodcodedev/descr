pub struct AstDef {
    pubident: String,
    pubtokenList: tokenList,
}

pub struct AstMany {
    pubident: String,
    pubastItems: astItems,
}

pub struct AstRef {
    pubident: String,
}

pub struct AstSingle {
    pubident: String,
    pubtokenList: tokenList,
}

pub struct TokenKey {
    puboptional: bool,
    pubident: String,
}

pub struct TokenNamedKey {
    puboptional: bool,
    pubname: String,
    pubkey: String,
}

pub enum AstItem {
    AstDefItem(AstDef),
    AstRefItem(AstRef),
}

pub enum Source {
    AstSingleItem(AstSingle),
    AstManyItem(AstMany),
    ListItem(List),
}

pub enum Token {
    TokenKeyItem(TokenKey),
    TokenNamedKeyItem(TokenNamedKey),
}

