pub struct TokenNamedKey {
    pub QUESTION: bool,
    pub ident: String,
    pub COLON: bool,
 }

pub struct AstRef {
    pub ident: String,
 }

pub struct TokenKey {
    pub QUESTION: bool,
    pub ident: String,
 }

pub struct AstDef {
    pub ident: String,
    pub LPAREN: bool,
    pub tokenList: tokenList,
    pub RPAREN: bool,
 }

pub struct AstMany {
    pub ident: String,
    pub RBRACE: bool,
    pub astItems: astItems,
    pub LBRACE: bool,
 }

pub struct AstSingle {
    pub LPAREN: bool,
    pub tokenList: tokenList,
    pub ident: String,
    pub RPAREN: bool,
 }

pub enum Source {
    AstSingleItem(AstSingle),
    AstManyItem(AstMany),
    ListItem(List),
}

pub enum AstItem {
    AstDefItem(AstDef),
    AstRefItem(AstRef),
}

pub enum Token {
    TokenKeyItem(TokenKey),
    TokenNamedKeyItem(TokenNamedKey),
}

