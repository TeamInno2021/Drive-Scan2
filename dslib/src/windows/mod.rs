mod scan;
mod verify;
mod winapi;

use super::File;
use std::path::{Path, PathBuf};

pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    verify::verify(dir)
}

pub fn scan(dir: PathBuf) -> Result<File, Box<dyn ::std::error::Error>> {
    scan::scan(dir.clone());
    Ok(File::File { path: dir, size: 0 })
}
