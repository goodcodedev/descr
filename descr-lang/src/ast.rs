#[derive(Debug)]
pub struct AstDef<'a> {
    pub ident: Option<&'a str>,
    pub tokens: Vec<Token<'a>>,
}

#[derive(Debug)]
pub struct AstMany<'a> {
    pub ident: &'a str,
    pub items: Vec<AstItem<'a>>,
}

#[derive(Debug)]
pub struct AstRef<'a> {
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct AstSingle<'a> {
    pub ident: &'a str,
    pub tokens: Vec<Token<'a>>,
}

#[derive(Debug)]
pub struct ListItem<'a> {
    pub ast_item: AstItem<'a>,
    pub ident: &'a str,
    pub sep: Option<&'a str>,
}

#[derive(Debug)]
pub struct ListMany<'a> {
    pub ident: &'a str,
    pub items: Vec<ListItem<'a>>,
    pub sep: Option<&'a str>,
}

#[derive(Debug)]
pub struct ListSingle<'a> {
    pub ident: &'a str,
    pub reference: &'a str,
    pub sep: &'a str,
}

#[derive(Debug)]
pub struct Source<'a> {
    pub items: Vec<SourceItem<'a>>,
}

#[derive(Debug)]
pub struct TokenKey<'a> {
    pub ident: &'a str,
    pub optional: bool,
}

#[derive(Debug)]
pub struct TokenNamedKey<'a> {
    pub key: &'a str,
    pub name: &'a str,
    pub optional: bool,
}

#[derive(Debug)]
pub enum AstItem {
    AstDefItem(AstDef),
    AstRefItem(AstRef),
}

#[derive(Debug)]
pub enum List {
    ListSingleItem(ListSingle),
    ListManyItem(ListMany),
}

#[derive(Debug)]
pub enum SourceItem {
    AstSingleItem(AstSingle),
    AstManyItem(AstMany),
    ListItem(List),
}

#[derive(Debug)]
pub enum Token {
    TokenKeyItem(TokenKey),
    TokenNamedKeyItem(TokenNamedKey),
}

