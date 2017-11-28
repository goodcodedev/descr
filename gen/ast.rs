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
    pub ident: String,
    pub tokenList: tokenList,
}

pub struct ListItem {
    pub ident: String,
    pub tokenList: tokenList,
    pub sep: String,
}

pub struct ListMany {
    pub sep: String,
    pub ListItem: ListItem,
    pub ident: String,
}

pub struct ListSingle {
    pub ident: String,
    pub sep: String,
    pub reference: String,
}

pub struct TokenKey {
    pub optional: bool,
    pub ident: String,
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

pub enum List {
    ListSingleItem(ListSingle),
    ListManyItem(ListMany),
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

