use std::ffi::OsString;
use std::fmt;

#[derive(Debug)]
pub struct OsError(pub OsString);

impl ::std::error::Error for OsError {}

impl From<&'static str> for OsError {
    fn from(s: &'static str) -> Self {
        s.to_string().into()
    }
}

impl From<String> for OsError {
    fn from(s: String) -> Self {
        OsError(s.into())
    }
}

impl From<OsString> for OsError {
    fn from(s: OsString) -> Self {
        OsError(s)
    }
}

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
