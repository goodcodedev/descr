use super::ast::*;

pub struct ToSource;
#[allow(unused_variables,dead_code)]
impl<'a> ToSource {
    pub fn to_source_int_const(mut s: String, node: &'a IntConst) -> String {
        s += " ";
        s += &node.int.to_string();
        s
    }

    pub fn to_source_token_group(mut s: String, node: &'a TokenGroup) -> String {
        s += " ";
        let len = node.annots.len();
        for (i, item) in node.annots.iter().enumerate() {
            s = Self::to_source_annotation(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        if node.not {
                s.push('!');
    }        s += " ";
        s.push('(');
        s += " ";
        let len = node.token_list.len();
        for (i, item) in node.token_list.iter().enumerate() {
            s = Self::to_source_token(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s.push(')');
        s += " ";
        if node.optional {
                s.push('?');
    }        s
    }

    pub fn to_source_named_token(mut s: String, node: &'a NamedToken) -> String {
        s += " ";
        let len = node.annots.len();
        for (i, item) in node.annots.iter().enumerate() {
            s = Self::to_source_annotation(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s += node.name;
        s += " ";
        s.push(':');
        s += " ";
        if node.not {
                s.push('!');
    }        s += " ";
        s = Self::to_source_token_type(s, &node.token_type);
        s += " ";
        if node.optional {
                s.push('?');
    }        s
    }

    pub fn to_source_comment(mut s: String, node: &'a Comment) -> String {
        s += " ";
        s += "(*";
        s += " ";
        s += node.comment;
        s += " ";
        s += "*)";
        s
    }

    pub fn to_source_ast_single(mut s: String, node: &'a AstSingle) -> String {
        s += " ";
        let len = node.annots.len();
        for (i, item) in node.annots.iter().enumerate() {
            s = Self::to_source_annotation(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s += node.ident;
        s += " ";
        s.push('(');
        s += " ";
        let len = node.tokens.len();
        for (i, item) in node.tokens.iter().enumerate() {
            s = Self::to_source_token(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s.push(')');
        s
    }

    pub fn to_source_list_item(mut s: String, node: &'a ListItem) -> String {
        s += " ";
        s = Self::to_source_ast_item(s, &node.ast_item);
        s += " ";
        if let Some(some_val) = node.sep {
                s += some_val;
        }        s
    }

    pub fn to_source_annot_args(mut s: String, node: &'a AnnotArgs) -> String {
        s += " ";
        s.push('(');
        s += " ";
        let len = node.annot_arg_list.len();
        for (i, item) in node.annot_arg_list.iter().enumerate() {
            s = Self::to_source_annot_arg(s, item);
            if i < len - 1 {         s.push(',');
 }
        }
        s += " ";
        s.push(')');
        s
    }

    pub fn to_source_source(mut s: String, node: &'a Source) -> String {
        s += " ";
        let len = node.items.len();
        for (i, item) in node.items.iter().enumerate() {
            s = Self::to_source_source_item(s, item);
            if i < len - 1 {         s += " " }
        }
        s
    }

    pub fn to_source_annot_arg(mut s: String, node: &'a AnnotArg) -> String {
        s += " ";
        s += node.key;
        s += " ";
        s.push('=');
        s += " ";
        s = Self::to_source_annot_arg_val(s, &node.annot_arg_val);
        s
    }

    pub fn to_source_quoted(mut s: String, node: &'a Quoted) -> String {
        s += " ";
        s += "\"";
        s += node.string;
        s += "\"";
        s += " ";
        s += "\"";
        s += node.string;
        s += "\"";
        s += " ";
        s += "\"";
        s += node.string;
        s += "\"";
        s
    }

    pub fn to_source_ident(mut s: String, node: &'a Ident) -> String {
        s += " ";
        s += node.ident;
        s
    }

    pub fn to_source_ast_many(mut s: String, node: &'a AstMany) -> String {
        s += " ";
        let len = node.annots.len();
        for (i, item) in node.annots.iter().enumerate() {
            s = Self::to_source_annotation(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s += node.ident;
        s += " ";
        s.push('{');
        s += " ";
        let len = node.items.len();
        for (i, item) in node.items.iter().enumerate() {
            s = Self::to_source_ast_item(s, item);
            if i < len - 1 {         s.push(',');
 }
        }
        s += " ";
        s.push('}');
        s
    }

    pub fn to_source_key_token(mut s: String, node: &'a KeyToken) -> String {
        s += " ";
        s += node.key;
        s
    }

    pub fn to_source_simple_token(mut s: String, node: &'a SimpleToken) -> String {
        s += " ";
        let len = node.annots.len();
        for (i, item) in node.annots.iter().enumerate() {
            s = Self::to_source_annotation(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        if node.not {
                s.push('!');
    }        s += " ";
        s = Self::to_source_token_type(s, &node.token_type);
        s += " ";
        if node.optional {
                s.push('?');
    }        s
    }

    pub fn to_source_ast_ref(mut s: String, node: &'a AstRef) -> String {
        s += " ";
        s += node.ident;
        s
    }

    pub fn to_source_list_many(mut s: String, node: &'a ListMany) -> String {
        s += " ";
        let len = node.annots.len();
        for (i, item) in node.annots.iter().enumerate() {
            s = Self::to_source_annotation(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s += node.ident;
        s += " ";
        s.push(':');
        s += " ";
        s += node.ast_type;
        s += " ";
        s.push('[');
        s += " ";
        s.push(']');
        s += " ";
        if let Some(some_val) = node.sep {
                s += some_val;
        }        s += " ";
        s.push('{');
        s += " ";
        let len = node.items.len();
        for (i, item) in node.items.iter().enumerate() {
            s = Self::to_source_list_item(s, item);
            if i < len - 1 {         s.push(',');
 }
        }
        s += " ";
        s.push('}');
        s
    }

    pub fn to_source_list_single(mut s: String, node: &'a ListSingle) -> String {
        s += " ";
        let len = node.annots.len();
        for (i, item) in node.annots.iter().enumerate() {
            s = Self::to_source_annotation(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s += node.ident;
        s += " ";
        s.push('[');
        s += " ";
        s.push(']');
        s += " ";
        s += node.sep;
        s += " ";
        s += node.reference;
        s
    }

    pub fn to_source_func_token(mut s: String, node: &'a FuncToken) -> String {
        s += " ";
        s += node.ident;
        s += " ";
        s.push('(');
        s += " ";
        let len = node.fn_args.len();
        for (i, item) in node.fn_args.iter().enumerate() {
            s = Self::to_source_func_arg(s, item);
            if i < len - 1 {         s.push(',');
 }
        }
        s += " ";
        s.push(')');
        s
    }

    pub fn to_source_ast_def(mut s: String, node: &'a AstDef) -> String {
        s += " ";
        let len = node.annots.len();
        for (i, item) in node.annots.iter().enumerate() {
            s = Self::to_source_annotation(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        let len = node.tokens.len();
        for (i, item) in node.tokens.iter().enumerate() {
            s = Self::to_source_token(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s += "=>";
        s += " ";
        if let Some(some_val) = node.ident {
                s += some_val;
        }        s += " ";
        let len = node.annots.len();
        for (i, item) in node.annots.iter().enumerate() {
            s = Self::to_source_annotation(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s.push('(');
        s += " ";
        let len = node.tokens.len();
        for (i, item) in node.tokens.iter().enumerate() {
            s = Self::to_source_token(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s.push(')');
        s += " ";
        s += "=>";
        s += " ";
        if let Some(some_val) = node.ident {
                s += some_val;
        }        s += " ";
        let len = node.annots.len();
        for (i, item) in node.annots.iter().enumerate() {
            s = Self::to_source_annotation(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        if let Some(some_val) = node.ident {
                s += some_val;
        }        s += " ";
        s.push('(');
        s += " ";
        let len = node.tokens.len();
        for (i, item) in node.tokens.iter().enumerate() {
            s = Self::to_source_token(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s.push(')');
        s
    }

    pub fn to_source_annotation(mut s: String, node: &'a Annotation) -> String {
        s += " ";
        s += "@";
        s += " ";
        s += node.ident;
        s += " ";
        if let Some(ref some_val) = node.annot_args {
            s = Self::to_source_annot_args(s, some_val);
        }
        s
    }

    pub fn to_source_annot_arg_val(s: String, node: &'a AnnotArgVal) -> String {
        match node {
            &AnnotArgVal::QuotedItem(ref inner) => Self::to_source_quoted(s, inner),
            &AnnotArgVal::IdentItem(ref inner) => Self::to_source_ident(s, inner),
            &AnnotArgVal::IntConstItem(ref inner) => Self::to_source_int_const(s, inner),
        }
    }

    pub fn to_source_ast_item(s: String, node: &'a AstItem) -> String {
        match node {
            &AstItem::AstDefItem(ref inner) => Self::to_source_ast_def(s, inner),
            &AstItem::AstRefItem(ref inner) => Self::to_source_ast_ref(s, inner),
        }
    }

    pub fn to_source_func_arg(s: String, node: &'a FuncArg) -> String {
        match node {
            &FuncArg::QuotedItem(ref inner) => Self::to_source_quoted(s, inner),
        }
    }

    pub fn to_source_list(s: String, node: &'a List) -> String {
        match node {
            &List::ListSingleItem(ref inner) => Self::to_source_list_single(s, inner),
            &List::ListManyItem(ref inner) => Self::to_source_list_many(s, inner),
        }
    }

    pub fn to_source_source_item(s: String, node: &'a SourceItem) -> String {
        match node {
            &SourceItem::AstSingleItem(ref inner) => Self::to_source_ast_single(s, inner),
            &SourceItem::AstManyItem(ref inner) => Self::to_source_ast_many(s, inner),
            &SourceItem::ListItem(ref inner) => Self::to_source_list(s, inner),
            &SourceItem::CommentItem(ref inner) => Self::to_source_comment(s, inner),
        }
    }

    pub fn to_source_token(s: String, node: &'a Token) -> String {
        match node {
            &Token::NamedTokenItem(ref inner) => Self::to_source_named_token(s, inner),
            &Token::SimpleTokenItem(ref inner) => Self::to_source_simple_token(s, inner),
            &Token::TokenGroupItem(ref inner) => Self::to_source_token_group(s, inner),
        }
    }

    pub fn to_source_token_type(s: String, node: &'a TokenType) -> String {
        match node {
            &TokenType::FuncTokenItem(ref inner) => Self::to_source_func_token(s, inner),
            &TokenType::KeyTokenItem(ref inner) => Self::to_source_key_token(s, inner),
            &TokenType::QuotedItem(ref inner) => Self::to_source_quoted(s, inner),
        }
    }

}