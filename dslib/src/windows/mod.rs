mod drive;
mod error;
mod filesystem;
mod mft;
mod re;
mod winapi;

use super::Directory;
use drive::DriveInfo;
use error::OsError;
// use re as mft;
// use mft::MftScanner;
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

pub fn scan(dir: PathBuf) -> Result<(), Box<dyn ::std::error::Error>> {
    debug!("Beginning scan of {:?}", dir);
    let drive = DriveInfo::parse(dir.clone())?;

    // let mut scanner = unsafe { MftScanner::init(drive)? };
    // unsafe { scanner.scan()? };

    let _nodes = mft::process(drive)?;

    Ok(())
}

pub fn query(_dir: PathBuf) -> Result<Option<Directory>, Box<dyn ::std::error::Error>> {
    Ok(None)
}
