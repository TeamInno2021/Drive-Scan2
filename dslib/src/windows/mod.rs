mod boot;
mod error;
mod filesystem;
mod winapi;

use super::Directory;
use error::OsError;
use std::mem::size_of;
use std::path::{Path, PathBuf};

pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    // Ensure we are running on a 64-bit system,
    // this allows us to enforce that `usize` is a 64-bit integer
    if size_of::<usize>() != size_of::<u64>() {
        Ok(false)
    } else if !dir.exists() {
        Err("target path does not exist".into())
    } else if !dir.is_dir() {
        Err("target path is not a valid directory".into())
    } else {
        Ok(filesystem::identify(dir)? == "NTFS")
    }
}

pub fn scan(_dir: PathBuf) -> Result<(), Box<dyn ::std::error::Error>> {
    Ok(())
}

pub fn query(_dir: PathBuf) -> Result<Option<Directory>, Box<dyn ::std::error::Error>> {
    Ok(None)
}
