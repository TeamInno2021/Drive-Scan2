use super::{File};
use std::path::{Path, PathBuf};

pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    Ok(false)
}

pub fn scan(dir: PathBuf) -> Result<(), Box<dyn ::std::error::Error>> {
    todo!();
}

pub fn query(dir: PathBuf) -> Result<Option<File>, Box<dyn ::std::error::Error>> {
    todo!();
}
