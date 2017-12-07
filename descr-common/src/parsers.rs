extern crate nom;
use self::nom::*;
use std::str;

// Quoted with " or single '
// then escapes quote, \ and n with \
named!(pub quoted_str<&str>, map_res!(alt_complete!(
    delimited!(
        tag!("\""),
        escaped!(
            is_not!("\\\""),
            '\\',
            one_of!("\"\\n")
        ), 
        tag!("\"")
    )
    | delimited!(
        tag!("'"), 
        escaped!(is_not!("\\'"), '\\', one_of!("\\n'")), 
        tag!("'")
    )
), str::from_utf8));

// Identifier starting with alpha
// or _, then alphanumeric + _
pub fn ident(input: &[u8]) -> IResult<&[u8], &str> {
    if input.len() == 0 {
        return IResult::Incomplete(Needed::Size(1));
    } else {
        let first = input[0];
        if first.is_alpha() || first == '_' as u8 {
            for (i, val) in input[1..].iter().enumerate() {
                if !(val.is_alphanum() || val.as_char() == '_') {
                    return IResult::Done(&input[i + 1..], str::from_utf8(&input[..i + 1]).unwrap());
                }
            }
            return IResult::Done(
                &input[input.len()..],
                str::from_utf8(&input[..input.len()]).unwrap(),
            );
        } else {
            return IResult::Error(error_code!(ErrorKind::Custom(42)));
        }
    }
}

pub fn parse_int(input: &[u8]) -> IResult<&[u8], u32> {
    let len = input.len();
    if len == 0 {
        return IResult::Incomplete(Needed::Size(1));
    } else {
        let first = input[0];
        let mut i = if is_digit(first) {
            1
        } else if first == '-' as u8 || first == '+' as u8 {
            if len < 2 || !is_digit(input[1]) {
                return IResult::Error(error_code!(ErrorKind::Digit));
            } else {
                2
            }
        } else {
            return IResult::Error(error_code!(ErrorKind::Digit));
        };
        while i < len && is_digit(input[i]) {
            i += 1;
        }
        let parsed = str::from_utf8(&input[..i]).unwrap().parse::<u32>().unwrap();
        return IResult::Done(&input[i..], parsed);
    }
}

// Some special chars that could possibly
// be parsed without quotes
pub fn special_chars(input: &[u8]) -> IResult<&[u8], &str> {
    if input.len() == 0 {
        return IResult::Incomplete(Needed::Size(1));
    } else {
        let mut i = 0;
        while i < input.len() {
            if !(input[i] == 0x21 // !, then skip quote (")
                || (input[i] >= 0x23 && input[i] <= 0x26) // # - &, then skip single quote
                || input[i] == 0x28 // (, then skip )
                || (input[i] >= 0x2A && input[i] <= 0x2F) // * - /
                || (input[i] >= 0x3A && input[i] <= 0x40) // : - @
                || (input[i] >= 0x5B && input[i] <= 0x60) // [ - `
                || (input[i] >= 0x7B && input[i] <= 0x7E))
            // { - ~
            {
                break;
            }
            i += 1;
        }
        if i > 0 {
            return IResult::Done(&input[i..], str::from_utf8(&input[0..i]).unwrap());
        } else {
            return IResult::Error(error_code!(ErrorKind::Custom(41)));
        }
    }
}

#[macro_export]
macro_rules! until_done_result (
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        let mut index = 0;
        let mut is_done = false;
        loop {
            let i_ = $i.slice(index..);
            match peek!(i_, $submac!($($args)* )) {
                IResult::Done(..) => {
                    is_done = true;
                    break;
                },
                _ => {}
            }
            index += 1;
            if index > $i.len() {
                break;
            }
        }
        if is_done && index > 0 {
            IResult::Done($i.slice(index..), $i.slice(..index))
        } else {
            IResult::Incomplete(Needed::Unknown)
        }
    });
);

#[macro_export]
macro_rules! stringify_submac (
    ($submac:ident( $($args:tt)* )) => ({
        let mut s = String::new();
        s += stringify!($submac);
        s += "!(";
        $(s += stringify!($args);)*
        s += ")";
        s
    });
    ($submac:ident) => ({
        stringify!($submac)
    })
);

#[macro_export]
macro_rules! debug_wrap (
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        let input = $i;
        let to = if input.len() > 10 { 10 } else { input.len() };
        use std;
        let start_str = std::str::from_utf8(&input[..to]).unwrap();
        println!("Starting {} at: {}...", stringify_submac!($submac($($args)*)), start_str);
        {
            let result = $submac!(input, $($args)*);
            match result {
                IResult::Done(ref inp, ref out) => {
                    let to = if inp.len() > 10 { 10 } else { inp.len() };
                    let done_str = std::str::from_utf8(&inp[..to]).unwrap();
                    println!("Done with {}: {:#?}, rest: {}...", stringify_submac!($submac($($args)*)), out, done_str);
                },
                IResult::Error(ref err) => {
                    println!("Error {}: {:#?}", stringify_submac!($submac($($args)*)), err);
                },
                IResult::Incomplete(ref inc) => {
                    println!("Incomplete {}: {:#?}", stringify_submac!($submac($($args)*)), inc);
                }
            }
            result
        }
    });
    ($i:expr, $submac:ident) => ({
        let input = $i;
        let to = if input.len() > 10 { 10 } else { input.len() };
        use std;
        let start_str = std::str::from_utf8(&input[..to]).unwrap();
        println!("Starting {} at: {}...", stringify!($submac), start_str);
        {
            let result = $submac(input);
            match result {
                IResult::Done(ref inp, ref out) => {
                    let to = if inp.len() > 10 { 10 } else { inp.len() };
                    let done_str = std::str::from_utf8(&inp[..to]).unwrap();
                    println!("Done with {}: {:#?}, rest: {}...", stringify!($submac), out, done_str);
                },
                IResult::Error(ref err) => {
                    println!("Error {}: {:#?}", stringify!($submac), err);
                },
                IResult::Incomplete(ref inc) => {
                    println!("Incomplete {}: {:#?}", stringify!($submac), inc);
                }
            }
            result
        }
    })
);
