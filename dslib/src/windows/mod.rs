mod boot;
mod drive;
mod error;
mod filesystem;
mod mft;
mod winapi;

use super::File;
use drive::DriveInfo;
use error::OsError;
use std::path::{Path, PathBuf};

pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    Ok(filesystem::identify(dir)? == "NTFS")
}

pub fn scan(dir: PathBuf) -> Result<File, Box<dyn ::std::error::Error>> {
    let drive = DriveInfo::parse(dir.clone())?;
    info!("Fetched metadata for {:#?}", drive);

    let _nodes = mft::process(drive)?;

    Ok(File::File { path: dir, size: 0 })
}
