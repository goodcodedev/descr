#[derive(Debug)]
pub struct EnumItem<'a> {
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct GenType<'a> {
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct Generic<'a> {
    pub generic_items: Vec<GenericItem<'a>>,
}

#[derive(Debug)]
pub struct LifeTime<'a> {
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct RsEnum<'a> {
    pub enum_items: Vec<EnumItem<'a>>,
    pub ident: &'a str,
    pub public: bool,
}

#[derive(Debug)]
pub struct RsStruct<'a> {
    pub generic: Option<Generic<'a>>,
    pub ident: &'a str,
    pub public: bool,
    pub struct_members: Vec<StructMember<'a>>,
}

#[derive(Debug)]
pub struct RsTrait<'a> {
    pub ident: &'a str,
    pub public: bool,
}

#[derive(Debug)]
pub struct Source<'a> {
    pub source_items: Vec<SourceItem<'a>>,
}

#[derive(Debug)]
pub struct StructMember<'a> {
    pub tpe_spes: TpeSpes<'a>,
    pub ident: &'a str,
    pub public: bool,
}

#[derive(Debug)]
pub struct TpeSpes<'a> {
    pub generic_item: Option<GenericItem<'a>>,
    pub tpe: Tpe,
}

#[derive(Debug)]
pub enum GenericItem<'a> {
    LifeTimeItem(LifeTime<'a>),
    GenTypeItem(GenType<'a>),
}

#[derive(Debug)]
pub enum SourceItem<'a> {
    RsStructItem(RsStruct<'a>),
    RsEnumItem(RsEnum<'a>),
    RsTraitItem(RsTrait<'a>),
}

#[derive(Debug)]
pub enum Tpe {
    RsU32,
    RsI32,
    RsString,
    RsStr,
    Bool,
}

