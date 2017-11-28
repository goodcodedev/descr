trait Visitor {
    pub fn visit_AstDef(node: AstDef) {
        self.ident match {
            Some(ref inner) => self.visit_ident(inner)),
            None => {}
        }
        self.visit_tokenList(node.tokenList);
    }

    pub fn visit_AstMany(node: AstMany) {
        self.visit_astItems(node.astItems);
        self.visit_ident(node.ident);
    }

    pub fn visit_AstRef(node: AstRef) {
        self.visit_ident(node.ident);
    }

    pub fn visit_AstSingle(node: AstSingle) {
        self.visit_ident(node.ident);
        self.visit_tokenList(node.tokenList);
    }

    pub fn visit_TokenKey(node: TokenKey) {
        self.visit_ident(node.ident);
        self.optional match {
            Some(ref inner) => self.visit_optional(inner)),
            None => {}
        }
    }

    pub fn visit_TokenNamedKey(node: TokenNamedKey) {
        self.visit_key(node.key);
        self.visit_name(node.name);
        self.optional match {
            Some(ref inner) => self.visit_optional(inner)),
            None => {}
        }
    }

    pub fn visit_AstItem(node: AstItem) {
        match node {
            AstDefItem(ref inner) => self.visit_AstDef(inner);
            AstRefItem(ref inner) => self.visit_AstRef(inner);
        }
    }

    pub fn visit_Source(node: Source) {
        match node {
            AstSingleItem(ref inner) => self.visit_AstSingle(inner);
            AstManyItem(ref inner) => self.visit_AstMany(inner);
            ListItem(ref inner) => self.visit_List(inner);
        }
    }

    pub fn visit_Token(node: Token) {
        match node {
            TokenKeyItem(ref inner) => self.visit_TokenKey(inner);
            TokenNamedKeyItem(ref inner) => self.visit_TokenNamedKey(inner);
        }
    }

}