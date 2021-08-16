use super::File;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    /// The cached data for the scanner
    static ref DATA: Arc<Mutex<Option<File>>> = Arc::new(Mutex::new(None));
}

#[inline]
pub fn store(data: File) {
    *DATA.lock().unwrap() = Some(data);
}

#[inline]
pub fn fetch() -> Arc<Mutex<Option<File>>> {
    DATA.clone()
}
