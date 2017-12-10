#[derive(Debug)]
pub struct ArrayVal<'a> {
    pub array_vals: Vec<JsVal<'a>>,
}

impl<'a> ArrayVal<'a> {
    pub fn new(array_vals: Vec<JsVal<'a>>) -> ArrayVal<'a> {
        ArrayVal {
            array_vals
        }
    }
}

#[derive(Debug)]
pub struct Int {
    pub int: u32,
}

impl Int {
    pub fn new(int: u32) -> Int {
        Int {
            int
        }
    }
}

#[derive(Debug)]
pub struct JsObject<'a> {
    pub items: Vec<ObjectPair<'a>>,
}

impl<'a> JsObject<'a> {
    pub fn new(items: Vec<ObjectPair<'a>>) -> JsObject<'a> {
        JsObject {
            items
        }
    }
}

#[derive(Debug)]
pub struct ObjectPair<'a> {
    pub key: &'a str,
    pub val: JsVal<'a>,
}

impl<'a> ObjectPair<'a> {
    pub fn new(key: &'a str, val: JsVal<'a>) -> ObjectPair<'a> {
        ObjectPair {
            key,
            val
        }
    }
}

#[derive(Debug)]
pub struct StringVal<'a> {
    pub string: &'a str,
}

impl<'a> StringVal<'a> {
    pub fn new(string: &'a str) -> StringVal<'a> {
        StringVal {
            string
        }
    }
}

#[derive(Debug)]
pub enum JsVal<'a> {
    IntItem(Int),
    StringValItem(StringVal<'a>),
    ArrayValItem(Box<ArrayVal<'a>>),
    JsObjectItem(Box<JsObject<'a>>),
}

impl<'a> JsVal<'a> {
    pub fn int(int: u32) -> JsVal<'a> {
        JsVal::IntItem(Int::new(int))
    }

    pub fn string_val(string: &'a str) -> JsVal<'a> {
        JsVal::StringValItem(StringVal::new(string))
    }

    pub fn array_val(array_vals: Vec<JsVal<'a>>) -> JsVal<'a> {
        JsVal::ArrayValItem(Box::new(ArrayVal::new(array_vals)))
    }

    pub fn js_object(items: Vec<ObjectPair<'a>>) -> JsVal<'a> {
        JsVal::JsObjectItem(Box::new(JsObject::new(items)))
    }
}

