mod alt;
mod boot;
mod drive;
mod error;
mod filesystem;
mod mft;
mod winapi;

use super::File;
use drive::DriveInfo;
use error::OsError;
use std::mem::size_of;
use std::path::{Path, PathBuf};

pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    // Make sure we aren't running on a 32-bit system (just in case)
    // This means we can enforce that `usize` is a 64 bit integer
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

pub fn scan(dir: PathBuf) -> Result<(), Box<dyn ::std::error::Error>> {
    // alt::scan(dir);

    let drive = DriveInfo::parse(dir.clone())?;
    let _nodes = mft::process(drive)?;
    // info!("{:#?}", nodes);
    Ok(())
}

pub fn query(_dir: PathBuf) -> Result<Option<File>, Box<dyn ::std::error::Error>> {
    Ok(None)
}
