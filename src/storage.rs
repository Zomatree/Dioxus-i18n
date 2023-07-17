
#[cfg(feature = "web")]
mod inner {
    use gloo::storage::{LocalStorage, Storage};
    use serde::{Serialize, Deserialize};

    pub fn set<K: AsRef<str>, V: Serialize>(key: K, value: V) {
        LocalStorage::set(key, value).unwrap()
    }

    pub fn get<K: AsRef<str>, V: for <'a> Deserialize<'a>>(key: K) -> Option<V> {
        LocalStorage::get(key).ok()
    }
}

#[cfg(feature = "desktop")]
mod inner {
    // future persistant desktop storage here
}

pub use inner::*;
