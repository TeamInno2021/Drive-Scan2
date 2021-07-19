//! win32api helper methods

use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};

/// Convert a rust str to a win32 lpstr, appending a null byte
pub fn to_wstring(val: &str) -> Vec<u16> {
    OsStr::new(val)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/// Call arbitary code with an automatic buffer for handling output values from wide winapi calls
pub fn winapi_call<F>(size: usize, f: F) -> OsString
where
    F: FnOnce(*mut u16),
{
    let mut str: Vec<u16> = Vec::with_capacity(size);
    f(str.as_mut_ptr());
    unsafe { str.set_len(size) };

    // remove any characters after the first null byte
    str.truncate(str.iter().position(|c| *c == 0).unwrap_or(str.len()));

    OsString::from_wide(&str)
}

/// Convert a rust string into a wide string pointer the winapi expects
pub fn winapi_str<S>(s: S) -> *const u16
where
    S: AsRef<str>,
{
    to_wstring(&s.as_ref()).as_ptr()
}
