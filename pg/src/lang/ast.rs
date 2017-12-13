#[derive(Debug)]
pub struct Test {
    pub key1: bool,
    pub key2: bool,
}

#[allow(dead_code)]
impl Test {
    pub fn new(key1: bool, key2: bool) -> Test {
        Test {
            key1,
            key2
        }
    }
}

