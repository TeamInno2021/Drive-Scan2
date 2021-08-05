use super::{File};
use std::path::PathBuf;

pub fn verify(_dir: &std::path::Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    Ok(true)
}

pub fn scan(dir: PathBuf) -> Result<(), Box<dyn ::std::error::Error>> {
    todo!();
}

pub fn query(dir: PathBuf) -> Result<Option<File>, Box<dyn ::std::error::Error>> {
    todo!();
}
