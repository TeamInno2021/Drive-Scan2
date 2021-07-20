mod drive;
mod error;
mod raw;
mod verify;
mod winapi;

use super::File;
use drive::DriveInfo;
use error::OsError;
use std::path::{Path, PathBuf};

pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    Ok(verify::verify(dir)?)
}

pub fn scan(dir: PathBuf) -> Result<File, Box<dyn ::std::error::Error>> {
    let drive = DriveInfo::parse(dir.clone())?;
    info!("Fetched metadata for {:#?}", drive);

    Ok(File::File { path: dir, size: 0 })
}
