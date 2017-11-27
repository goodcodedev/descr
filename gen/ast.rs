pub struct AstDef {
    pub LPAREN: bool,
    pub RPAREN: bool,
    pub ident: String,
    pub tokenList: tokenList,
 }

pub struct AstMany {
    pub astItems: astItems,
    pub ident: String,
    pub RBRACE: bool,
    pub LBRACE: bool,
 }

pub struct AstRef {
    pub ident: String,
 }

pub struct AstSingle {
    pub tokenList: tokenList,
    pub RPAREN: bool,
    pub ident: String,
    pub LPAREN: bool,
 }

pub struct TokenKey {
    pub ident: String,
    pub QUESTION: bool,
 }

pub struct TokenNamedKey {
    pub QUESTION: bool,
    pub ident: String,
    pub COLON: bool,
 }

pub enum Token {
    TokenKeyItem(TokenKey),
    TokenNamedKeyItem(TokenNamedKey),
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

