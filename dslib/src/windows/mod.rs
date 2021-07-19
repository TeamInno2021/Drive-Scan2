mod verify;

use super::File;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};

/// Convert a rust str to a win32 lpstr, appending a null byte
fn to_wstring(val: &str) -> Vec<u16> {
    OsStr::new(val)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    verify::verify(dir)
}

pub fn scan(dir: PathBuf) -> Result<File, Box<dyn ::std::error::Error>> {
    Ok(File::File { path: dir, size: 0 })
}
