use super::File;
use std::path::{Path, PathBuf};

pub fn verify(_dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    Ok(false)
}

pub fn scan(_dir: PathBuf) -> Result<(), Box<dyn ::std::error::Error>> {
    unimplemented!();
}

pub fn query(_dir: PathBuf) -> Result<Option<File>, Box<dyn ::std::error::Error>> {
    unimplemented!();
}
