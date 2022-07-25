use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
};

#[derive(Clone)]
pub struct Dict<T> {
    inner: BTreeMap<String, T>,
}

impl<T> Debug for Dict<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut w = &mut f.debug_struct("");
        for (k, v) in self.inner.iter() {
            w = w.field(k, v)
        }
        w.finish()
    }
}

impl<T> Default for Dict<T> {
    fn default() -> Self {
        Self { inner: BTreeMap::new() }
    }
}

impl<T> Dict<T> {
    pub fn get(&self, key: &str) -> Option<&T> {
        self.inner.get(key)
    }
    pub fn put<S>(&mut self, key: S, value: T)
    where
        S: Into<String>,
    {
        self.inner.insert(key.into(), value);
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
    return out;
}