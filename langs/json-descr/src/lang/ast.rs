#[derive(Debug)]
pub struct ArrayVal<'a> {
    pub array_vals: Vec<JsVal<'a>>,
}

#[derive(Debug)]
pub struct Int {
    pub int: u32,
}

#[derive(Debug)]
pub struct JsObject<'a> {
    pub items: Vec<ObjectPair<'a>>,
}

#[derive(Debug)]
pub struct ObjectPair<'a> {
    pub js_val: JsVal<'a>,
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct StringVal<'a> {
    pub string: &'a str,
}

#[derive(Debug)]
pub enum JsVal<'a> {
    IntItem(Int),
    StringValItem(StringVal<'a>),
    ArrayValItem(Box<ArrayVal<'a>>),
    JsObjectItem(Box<JsObject<'a>>),
}

