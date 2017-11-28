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
        self.keys.sort();
        if self.i < self.keys.len() {
            let next_key = self.keys[self.i];
            self.i += 1;
            Some((next_key, self.inner.get(next_key).unwrap()))
        } else {
            None
        }
    }
}