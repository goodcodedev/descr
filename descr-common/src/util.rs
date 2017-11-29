use std;
use std::collections::HashMap;

pub trait SortedHashMap<K, V> {
    fn sorted_iter(&self) -> SortedHashMapIter<K, V>;
}

impl<K, V> SortedHashMap<K, V> for HashMap<K, V>
    where K: std::cmp::Eq + std::hash::Hash + std::cmp::Ord
{
    fn sorted_iter(&self) -> SortedHashMapIter<K, V> {
        SortedHashMapIter {
            inner: self,
            keys: self.keys().collect::<Vec<_>>(),
            i: 0
        }
    }
}

pub struct SortedHashMapIter<'a, K: 'a, V: 'a> {
    inner: &'a HashMap<K, V>,
    keys: Vec<&'a K>,
    i: usize
}
impl<'a, K: 'a, V: 'a> Iterator for SortedHashMapIter<'a, K, V> 
    where K: std::cmp::Eq + std::hash::Hash + std::cmp::Ord
{
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.keys.sort();
        }
        if self.i < self.keys.len() {
            let next_key = self.keys[self.i];
            self.i += 1;
            Some((next_key, self.inner.get(next_key).unwrap()))
        } else {
            None
        }
    }
}

pub fn load_file(filename: &str) -> Vec<u8> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut f = File::open(filename).expect(format!("Could not open file: {}", filename).as_str());
    let mut buf = Vec::with_capacity(1024);
    f.read_to_end(&mut buf).expect(format!("Could not read file: {}", filename).as_str());
    buf
}

#[macro_export]
macro_rules! append {
    ($s:ident, $($a:expr)*) => {
        $($s += $a;)*
    };
    ($s:ident 1, $($a:expr)*) => {
        $s += "    ";
        $($s += $a;)*
    };
    ($s:ident 2, $($a:expr)*) => {
        $s += "        ";
        $($s += $a;)*
    };
    ($s:ident 3, $($a:expr)*) => {
        $s += "            ";
        $($s += $a;)*
    };
    ($s:ident 4, $($a:expr)*) => {
        $s += "                ";
        $($s += $a;)*
    };
    ($s:ident 5, $($a:expr)*) => {
        $s += "                    ";
        $($s += $a;)*
    };
    ($s:ident 6, $($a:expr)*) => {
        $s += "                        ";
        $($s += $a;)*
    };
    ($s:ident 7, $($a:expr)*) => {
        $s += "                            ";
        $($s += $a;)*
    };
    ($s:ident 8, $($a:expr)*) => {
        $s += "                                ";
        $($s += $a;)*
    };
}

#[macro_export]
macro_rules! indent {
    ($s:ident $indent:expr) => {
        match $indent {
            1 => $s += "    ",
            2 => $s += "        ",
            3 => $s += "            ",
            4 => $s += "                ",
            5 => $s += "                    ",
            6 => $s += "                        ",
            7 => $s += "                            ",
            8 => $s += "                                ",
            0 => {},
            num => {
                for _i in 0..num {
                    $s += "    ";
                }
            }
        }
    };
}

#[macro_export]
macro_rules! measure {
    ($name:expr, $b:block) => {{
        let (elapsed, ()) = measure_time(|| $b);
        println!("{}: {}", $name, elapsed);
    }};
}