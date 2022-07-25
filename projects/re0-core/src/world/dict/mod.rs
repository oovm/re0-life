use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone)]
pub struct Dict<T> {
    inner: BTreeMap<String, T>
}

impl<T> Debug for Dict<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

    }
}




impl<T> Dict<T> {
    pub fn get(&self, key: &str) -> Option<&T> {
        self.inner.get(key)
    }
    pub fn get_one_of(&self, keys: &[&str]) -> Option<&T> {
        for key in keys {
            if let Some(v) = self.get(key) {
                return Some(v);
            }
        }
        None
    }
}

#[inline]
pub fn get_flatten_vec<T: Clone>(data: &Dict<Vec<T>>, key: &[&str]) -> Vec<T> {
    let mut out = vec![];
    for k in key {
        match data.get(k) {
            Some(v) => {
                out.extend(v.iter().cloned());
            }
            None => {}
        }
    }
    return out
}