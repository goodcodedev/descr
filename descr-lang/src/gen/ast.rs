#[derive(Debug)]
pub struct AnnotArg<'a> {
    pub annot_arg_val: AnnotArgVal<'a>,
    pub key: &'a str,
}

#[allow(dead_code)]
impl<'a> AnnotArg<'a> {
    pub fn new(key: &'a str, annot_arg_val: AnnotArgVal<'a>) -> AnnotArg<'a> {
        AnnotArg {
            key,
            annot_arg_val
        }
    }
}

#[derive(Debug)]
pub struct AnnotArgs<'a> {
    pub annot_arg_list: Vec<AnnotArg<'a>>,
}

#[allow(dead_code)]
impl<'a> AnnotArgs<'a> {
    pub fn new(annot_arg_list: Vec<AnnotArg<'a>>) -> AnnotArgs<'a> {
        AnnotArgs {
            annot_arg_list
        }
    }
}

#[derive(Debug)]
pub struct Annotation<'a> {
    pub annot_args: Option<AnnotArgs<'a>>,
    pub ident: &'a str,
}

#[allow(dead_code)]
impl<'a> Annotation<'a> {
    pub fn new(ident: &'a str, annot_args: Option<AnnotArgs<'a>>) -> Annotation<'a> {
        Annotation {
            ident,
            annot_args
        }
    }
}

#[derive(Debug)]
pub struct AstDef<'a> {
    pub annots: Vec<Annotation<'a>>,
    pub ident: Option<&'a str>,
    pub tokens: Vec<Token<'a>>,
}

#[allow(dead_code)]
impl<'a> AstDef<'a> {
    pub fn new(annots: Vec<Annotation<'a>>, tokens: Vec<Token<'a>>, ident: Option<&'a str>) -> AstDef<'a> {
        AstDef {
            annots,
            tokens,
            ident
        }
    }

    pub fn as_ast_item(self) -> AstItem<'a> {
        AstItem::AstDefItem(self)
    }
}

#[derive(Debug)]
pub struct AstMany<'a> {
    pub annots: Vec<Annotation<'a>>,
    pub ident: &'a str,
    pub items: Vec<AstItem<'a>>,
}

#[allow(dead_code)]
impl<'a> AstMany<'a> {
    pub fn new(annots: Vec<Annotation<'a>>, ident: &'a str, items: Vec<AstItem<'a>>) -> AstMany<'a> {
        AstMany {
            annots,
            ident,
            items
        }
    }

    pub fn as_source_item(self) -> SourceItem<'a> {
        SourceItem::AstManyItem(self)
    }
}

#[derive(Debug)]
pub struct AstRef<'a> {
    pub ident: &'a str,
}

#[allow(dead_code)]
impl<'a> AstRef<'a> {
    pub fn new(ident: &'a str) -> AstRef<'a> {
        AstRef {
            ident
        }
    }

    pub fn as_ast_item(self) -> AstItem<'a> {
        AstItem::AstRefItem(self)
    }
}

#[derive(Debug)]
pub struct AstSingle<'a> {
    pub annots: Vec<Annotation<'a>>,
    pub ident: &'a str,
    pub tokens: Vec<Token<'a>>,
}

#[allow(dead_code)]
impl<'a> AstSingle<'a> {
    pub fn new(annots: Vec<Annotation<'a>>, ident: &'a str, tokens: Vec<Token<'a>>) -> AstSingle<'a> {
        AstSingle {
            annots,
            ident,
            tokens
        }
    }

    pub fn as_source_item(self) -> SourceItem<'a> {
        SourceItem::AstSingleItem(self)
    }
}

#[derive(Debug)]
pub struct Comment<'a> {
    pub comment: &'a str,
}

#[allow(dead_code)]
impl<'a> Comment<'a> {
    pub fn new(comment: &'a str) -> Comment<'a> {
        Comment {
            comment
        }
    }

    pub fn as_source_item(self) -> SourceItem<'a> {
        SourceItem::CommentItem(self)
    }
}

#[derive(Debug)]
pub struct FuncToken<'a> {
    pub fn_args: Vec<FuncArg<'a>>,
    pub ident: &'a str,
}

#[allow(dead_code)]
impl<'a> FuncToken<'a> {
    pub fn new(ident: &'a str, fn_args: Vec<FuncArg<'a>>) -> FuncToken<'a> {
        FuncToken {
            ident,
            fn_args
        }
    }

    pub fn as_token_type(self) -> TokenType<'a> {
        TokenType::FuncTokenItem(self)
    }
}

#[derive(Debug)]
pub struct Ident<'a> {
    pub ident: &'a str,
}

#[allow(dead_code)]
impl<'a> Ident<'a> {
    pub fn new(ident: &'a str) -> Ident<'a> {
        Ident {
            ident
        }
    }

    pub fn as_annot_arg_val(self) -> AnnotArgVal<'a> {
        AnnotArgVal::IdentItem(self)
    }
}

#[derive(Debug)]
pub struct IntConst {
    pub int: u32,
}

#[allow(dead_code)]
impl IntConst {
    pub fn new(int: u32) -> IntConst {
        IntConst {
            int
        }
    }

    pub fn as_annot_arg_val<'a>(self) -> AnnotArgVal<'a> {
        AnnotArgVal::IntConstItem(self)
    }
}

#[derive(Debug)]
pub struct KeyToken<'a> {
    pub key: &'a str,
}

#[allow(dead_code)]
impl<'a> KeyToken<'a> {
    pub fn new(key: &'a str) -> KeyToken<'a> {
        KeyToken {
            key
        }
    }

    pub fn as_token_type(self) -> TokenType<'a> {
        TokenType::KeyTokenItem(self)
    }
}

#[derive(Debug)]
pub struct ListItem<'a> {
    pub ast_item: AstItem<'a>,
    pub sep: Option<&'a str>,
}

#[allow(dead_code)]
impl<'a> ListItem<'a> {
    pub fn new(ast_item: AstItem<'a>, sep: Option<&'a str>) -> ListItem<'a> {
        ListItem {
            ast_item,
            sep
        }
    }
}

#[derive(Debug)]
pub struct ListMany<'a> {
    pub annots: Vec<Annotation<'a>>,
    pub ast_type: &'a str,
    pub ident: &'a str,
    pub items: Vec<ListItem<'a>>,
    pub sep: Option<&'a str>,
}

#[allow(dead_code)]
impl<'a> ListMany<'a> {
    pub fn new(annots: Vec<Annotation<'a>>, ident: &'a str, ast_type: &'a str, sep: Option<&'a str>, items: Vec<ListItem<'a>>) -> ListMany<'a> {
        ListMany {
            annots,
            ident,
            ast_type,
            sep,
            items
        }
    }

    pub fn as_list(self) -> List<'a> {
        List::ListManyItem(self)
    }
}

#[derive(Debug)]
pub struct ListSingle<'a> {
    pub annots: Vec<Annotation<'a>>,
    pub ident: &'a str,
    pub reference: &'a str,
    pub sep: &'a str,
}

#[allow(dead_code)]
impl<'a> ListSingle<'a> {
    pub fn new(annots: Vec<Annotation<'a>>, ident: &'a str, sep: &'a str, reference: &'a str) -> ListSingle<'a> {
        ListSingle {
            annots,
            ident,
            sep,
            reference
        }
    }

    pub fn as_list(self) -> List<'a> {
        List::ListSingleItem(self)
    }
}

#[derive(Debug)]
pub struct NamedToken<'a> {
    pub token_type: TokenType<'a>,
    pub annots: Vec<Annotation<'a>>,
    pub name: &'a str,
    pub not: bool,
    pub optional: bool,
}

#[allow(dead_code)]
impl<'a> NamedToken<'a> {
    pub fn new(annots: Vec<Annotation<'a>>, name: &'a str, not: bool, token_type: TokenType<'a>, optional: bool) -> NamedToken<'a> {
        NamedToken {
            annots,
            name,
            not,
            token_type,
            optional
        }
    }

    pub fn as_token(self) -> Token<'a> {
        Token::NamedTokenItem(self)
    }
}

#[derive(Debug)]
pub struct Quoted<'a> {
    pub string: &'a str,
}

#[allow(dead_code)]
impl<'a> Quoted<'a> {
    pub fn new(string: &'a str) -> Quoted<'a> {
        Quoted {
            string
        }
    }

    pub fn as_token_type(self) -> TokenType<'a> {
        TokenType::QuotedItem(self)
    }

    pub fn as_func_arg(self) -> FuncArg<'a> {
        FuncArg::QuotedItem(self)
    }

    pub fn as_annot_arg_val(self) -> AnnotArgVal<'a> {
        AnnotArgVal::QuotedItem(self)
    }
}

#[derive(Debug)]
pub struct SimpleToken<'a> {
    pub token_type: TokenType<'a>,
    pub annots: Vec<Annotation<'a>>,
    pub not: bool,
    pub optional: bool,
}

#[allow(dead_code)]
impl<'a> SimpleToken<'a> {
    pub fn new(annots: Vec<Annotation<'a>>, not: bool, token_type: TokenType<'a>, optional: bool) -> SimpleToken<'a> {
        SimpleToken {
            annots,
            not,
            token_type,
            optional
        }
    }

    pub fn as_token(self) -> Token<'a> {
        Token::SimpleTokenItem(self)
    }
}

#[derive(Debug)]
pub struct Source<'a> {
    pub items: Vec<SourceItem<'a>>,
}

#[allow(dead_code)]
impl<'a> Source<'a> {
    pub fn new(items: Vec<SourceItem<'a>>) -> Source<'a> {
        Source {
            items
        }
    }
}

#[derive(Debug)]
pub struct TokenGroup<'a> {
    pub annots: Vec<Annotation<'a>>,
    pub not: bool,
    pub optional: bool,
    pub token_list: Vec<Token<'a>>,
}

#[allow(dead_code)]
impl<'a> TokenGroup<'a> {
    pub fn new(annots: Vec<Annotation<'a>>, not: bool, token_list: Vec<Token<'a>>, optional: bool) -> TokenGroup<'a> {
        TokenGroup {
            annots,
            not,
            token_list,
            optional
        }
    }

    pub fn as_token(self) -> Token<'a> {
        Token::TokenGroupItem(self)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum AnnotArgVal<'a> {
    QuotedItem(Quoted<'a>),
    IdentItem(Ident<'a>),
    IntConstItem(IntConst),
}

impl<'a> AnnotArgVal<'a> {
    pub fn quoted(string: &'a str) -> AnnotArgVal<'a> {
        AnnotArgVal::QuotedItem(Quoted::new(string))
    }

    pub fn ident(ident: &'a str) -> AnnotArgVal<'a> {
        AnnotArgVal::IdentItem(Ident::new(ident))
    }

    pub fn int_const(int: u32) -> AnnotArgVal<'a> {
        AnnotArgVal::IntConstItem(IntConst::new(int))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum AstItem<'a> {
    AstDefItem(AstDef<'a>),
    AstRefItem(AstRef<'a>),
}

impl<'a> AstItem<'a> {
    pub fn ast_def(annots: Vec<Annotation<'a>>, tokens: Vec<Token<'a>>, ident: Option<&'a str>) -> AstItem<'a> {
        AstItem::AstDefItem(AstDef::new(annots, tokens, ident))
    }

    pub fn ast_ref(ident: &'a str) -> AstItem<'a> {
        AstItem::AstRefItem(AstRef::new(ident))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FuncArg<'a> {
    QuotedItem(Quoted<'a>),
}

impl<'a> FuncArg<'a> {
    pub fn quoted(string: &'a str) -> FuncArg<'a> {
        FuncArg::QuotedItem(Quoted::new(string))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum List<'a> {
    ListSingleItem(ListSingle<'a>),
    ListManyItem(ListMany<'a>),
}

impl<'a> List<'a> {
    pub fn list_single(annots: Vec<Annotation<'a>>, ident: &'a str, sep: &'a str, reference: &'a str) -> List<'a> {
        List::ListSingleItem(ListSingle::new(annots, ident, sep, reference))
    }

    pub fn list_many(annots: Vec<Annotation<'a>>, ident: &'a str, ast_type: &'a str, sep: Option<&'a str>, items: Vec<ListItem<'a>>) -> List<'a> {
        List::ListManyItem(ListMany::new(annots, ident, ast_type, sep, items))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum SourceItem<'a> {
    AstSingleItem(AstSingle<'a>),
    AstManyItem(AstMany<'a>),
    ListItem(List<'a>),
    CommentItem(Comment<'a>),
}

impl<'a> SourceItem<'a> {
    pub fn ast_single(annots: Vec<Annotation<'a>>, ident: &'a str, tokens: Vec<Token<'a>>) -> SourceItem<'a> {
        SourceItem::AstSingleItem(AstSingle::new(annots, ident, tokens))
    }

    pub fn ast_many(annots: Vec<Annotation<'a>>, ident: &'a str, items: Vec<AstItem<'a>>) -> SourceItem<'a> {
        SourceItem::AstManyItem(AstMany::new(annots, ident, items))
    }

    pub fn comment(comment: &'a str) -> SourceItem<'a> {
        SourceItem::CommentItem(Comment::new(comment))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Token<'a> {
    NamedTokenItem(NamedToken<'a>),
    SimpleTokenItem(SimpleToken<'a>),
    TokenGroupItem(TokenGroup<'a>),
}

impl<'a> Token<'a> {
    pub fn named_token(annots: Vec<Annotation<'a>>, name: &'a str, not: bool, token_type: TokenType<'a>, optional: bool) -> Token<'a> {
        Token::NamedTokenItem(NamedToken::new(annots, name, not, token_type, optional))
    }

    pub fn simple_token(annots: Vec<Annotation<'a>>, not: bool, token_type: TokenType<'a>, optional: bool) -> Token<'a> {
        Token::SimpleTokenItem(SimpleToken::new(annots, not, token_type, optional))
    }

    pub fn token_group(annots: Vec<Annotation<'a>>, not: bool, token_list: Vec<Token<'a>>, optional: bool) -> Token<'a> {
        Token::TokenGroupItem(TokenGroup::new(annots, not, token_list, optional))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum TokenType<'a> {
    FuncTokenItem(FuncToken<'a>),
    KeyTokenItem(KeyToken<'a>),
    QuotedItem(Quoted<'a>),
}

impl<'a> TokenType<'a> {
    pub fn func_token(ident: &'a str, fn_args: Vec<FuncArg<'a>>) -> TokenType<'a> {
        TokenType::FuncTokenItem(FuncToken::new(ident, fn_args))
    }

    pub fn key_token(key: &'a str) -> TokenType<'a> {
        TokenType::KeyTokenItem(KeyToken::new(key))
    }

    pub fn quoted(string: &'a str) -> TokenType<'a> {
        TokenType::QuotedItem(Quoted::new(string))
    }
}

