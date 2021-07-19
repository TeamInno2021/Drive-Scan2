use std::ffi::OsString;
use std::fmt;

/// A generic error wrapper around an OsString
#[derive(Debug, Clone)]
pub struct Error(OsString);

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error(s.into())
    }
}

impl From<OsString> for Error {
    fn from(s: OsString) -> Self {
        Error(s)
    }
}

impl<'s> From<&'s str> for Error {
    fn from(s: &'s str) -> Self {
        Error(s.to_string().into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

impl ::std::error::Error for Error {}
