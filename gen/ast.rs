pub struct TokenKey {
    pub ident: String,
    pub QUESTION: bool,
 }

pub struct AstDef {
    pub ident: String,
    pub tokenList: tokenList,
    pub LPAREN: bool,
    pub RPAREN: bool,
 }

pub struct AstRef {
    pub ident: String,
 }

pub struct TokenNamedKey {
    pub COLON: bool,
    pub QUESTION: bool,
    pub ident: String,
 }

pub struct AstMany {
    pub RBRACE: bool,
    pub ident: String,
    pub astItems: astItems,
    pub LBRACE: bool,
 }

pub struct AstSingle {
    pub LPAREN: bool,
    pub ident: String,
    pub RPAREN: bool,
    pub tokenList: tokenList,
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

