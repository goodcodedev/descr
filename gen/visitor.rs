trait Visitor {
    pub fn visit_AstDef(node:AstDef) {
        for item in &self.tokenList{
            self.visit_tokenList(item);
        }
    }

    pub fn visit_AstMany(node:AstMany) {
        for item in &self.astItems{
            self.visit_astItems(item);
        }
    }

    pub fn visit_AstRef(node:AstRef) {
    }

    pub fn visit_AstSingle(node:AstSingle) {
        for item in &self.tokenList{
            self.visit_tokenList(item);
        }
    }

    pub fn visit_TokenKey(node:TokenKey) {
    }

    pub fn visit_TokenNamedKey(node:TokenNamedKey) {
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