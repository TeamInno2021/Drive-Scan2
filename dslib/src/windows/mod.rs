mod error;
mod verify;
mod winapi;

use super::File;
use error::OsError;
use std::path::{Path, PathBuf};

pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    Ok(verify::verify(dir).map_err(|e| OsError(e))?)
}

pub fn scan(_dir: PathBuf) -> Result<File, Box<dyn ::std::error::Error>> {
    todo!();
}
