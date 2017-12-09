use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use std;
use super::ast::*;

named!(pub start<JsObject>, do_parse!(res: js_object >> (res)));

named!(pub js_object<JsObject>,
    do_parse!(
        sp >> char!('{') >>
        sp >> items_k: object_pairs >>
        sp >> char!('}') >>
        (JsObject {
            items: items_k,
        }))
);

named!(pub js_val<JsVal>, alt_complete!(
    do_parse!(
        sp >> int_k: parse_int >>
        (JsVal::IntItem(Int {
            int: int_k,
        })))
    | do_parse!(
        sp >> string_k: quoted_str >>
        (JsVal::StringValItem(StringVal {
            string: string_k,
        })))
    | do_parse!(
        sp >> char!('[') >>
        sp >> array_vals_k: array_vals >>
        sp >> char!(']') >>
        (JsVal::ArrayValItem(Box::new(ArrayVal {
            array_vals: array_vals_k,
        }))))
    | map!(js_object, |node| { JsVal::JsObjectItem(Box::new(node)) })
));

named!(pub array_vals<Vec<JsVal>>, separated_list!(char!(','), 
    js_val
));

named!(pub object_pairs<Vec<ObjectPair>>, separated_list!(char!(','), 
    do_parse!(
        sp >> key_k: quoted_str >>
        sp >> char!(':') >>
        sp >> js_val_k: js_val >>
        (ObjectPair {
            key: key_k,
            js_val: js_val_k,
        }))
));

