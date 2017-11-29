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
pub enum AstItem<'a> {
    AstDefItem(AstDef<'a>),
    AstRefItem(AstRef<'a>),
}

#[derive(Debug)]
pub enum List<'a> {
    ListSingleItem(ListSingle<'a>),
    ListManyItem(ListMany<'a>),
}

#[derive(Debug)]
pub enum SourceItem<'a> {
    AstSingleItem(AstSingle<'a>),
    AstManyItem(AstMany<'a>),
    ListItem(List<'a>),
}

#[derive(Debug)]
pub enum Token<'a> {
    TokenNamedKeyItem(TokenNamedKey<'a>),
    TokenKeyItem(TokenKey<'a>),
}

