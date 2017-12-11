#[derive(Debug)]
pub struct ArrayVal {
    pub items: Vec<JsVal>,
}

impl ArrayVal {
    pub fn new(items: Vec<JsVal>) -> ArrayVal {
        ArrayVal {
            items
        }
    }

    pub fn as_js_val(self) -> JsVal {
        JsVal::ArrayValItem(Box::new(self))
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

    pub fn as_js_val(self) -> JsVal {
        JsVal::IntItem(self)
    }
}

#[derive(Debug)]
pub struct JsObject {
    pub items: Vec<ObjectPair>,
}

impl JsObject {
    pub fn new(items: Vec<ObjectPair>) -> JsObject {
        JsObject {
            items
        }
    }

    pub fn as_js_val(self) -> JsVal {
        JsVal::JsObjectItem(Box::new(self))
    }
}

#[derive(Debug)]
pub struct ObjectPair {
    pub key: String,
    pub val: JsVal,
}

impl ObjectPair {
    pub fn new(key: String, val: JsVal) -> ObjectPair {
        ObjectPair {
            key,
            val
        }
    }
}

#[derive(Debug)]
pub struct StringVal {
    pub string: String,
}

impl StringVal {
    pub fn new(string: String) -> StringVal {
        StringVal {
            string
        }
    }

    pub fn as_js_val(self) -> JsVal {
        JsVal::StringValItem(self)
    }
}

#[derive(Debug)]
pub enum JsVal {
    IntItem(Int),
    StringValItem(StringVal),
    ArrayValItem(Box<ArrayVal>),
    JsObjectItem(Box<JsObject>),
}

impl JsVal {
    pub fn int(int: u32) -> JsVal {
        JsVal::IntItem(Int::new(int))
    }

    pub fn string_val(string: String) -> JsVal {
        JsVal::StringValItem(StringVal::new(string))
    }

    pub fn array_val(items: Vec<JsVal>) -> JsVal {
        JsVal::ArrayValItem(Box::new(ArrayVal::new(items)))
    }

    pub fn js_object(items: Vec<ObjectPair>) -> JsVal {
        JsVal::JsObjectItem(Box::new(JsObject::new(items)))
    }
}

