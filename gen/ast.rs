pub struct AstDef {
    pub tokenList: tokenList,
    pub ident: String,
}

pub struct AstMany {
    pub astItems: astItems,
    pub ident: String,
}

pub struct AstRef {
    pub ident: String,
}

pub struct AstSingle {
    pub tokenList: tokenList,
    pub ident: String,
}

pub struct TokenKey {
    pub ident: String,
    pub optional: bool,
}

pub struct TokenNamedKey {
    pub key: String,
    pub name: String,
    pub optional: bool,
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

