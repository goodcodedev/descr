pub struct AstDef {
    pub tokenList: tokenList,
    pub ident: String,
}

pub struct AstMany {
    pub ident: String,
    pub astItems: astItems,
}

pub struct AstRef {
    pub ident: String,
}

pub struct AstSingle {
    pub tokenList: tokenList,
    pub ident: String,
}

pub struct TokenKey {
    pub optional: bool,
    pub ident: String,
}

pub struct TokenNamedKey {
    pub key: String,
    pub optional: bool,
    pub name: String,
}

pub enum Token {
    TokenKeyItem(TokenKey),
    TokenNamedKeyItem(TokenNamedKey),
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

