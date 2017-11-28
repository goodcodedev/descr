pub struct AstDef<'a> {
    pub ident: Option<&'a str>,
    pub tokens: Vec<Token<'a>>,
}

pub struct AstMany<'a> {
    pub ident: &'a str,
    pub items: Vec<AstItem<'a>>,
}

pub struct AstRef<'a> {
    pub ident: &'a str,
}

pub struct AstSingle<'a> {
    pub tokens: Vec<Token<'a>>,
    pub ident: &'a str,
}

pub struct ListItem<'a> {
    pub sep: Option<&'a str>,
    pub ident: &'a str,
    pub ast_item: AstItem<'a>,
}

pub struct ListMany<'a> {
    pub sep: Option<&'a str>,
    pub items: Vec<ListItem<'a>>,
    pub ident: &'a str,
}

pub struct ListSingle<'a> {
    pub reference: &'a str,
    pub ident: &'a str,
    pub sep: &'a str,
}

pub struct TokenKey<'a> {
    pub optional: bool,
    pub ident: &'a str,
}

pub struct TokenNamedKey<'a> {
    pub optional: bool,
    pub name: &'a str,
    pub key: &'a str,
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

