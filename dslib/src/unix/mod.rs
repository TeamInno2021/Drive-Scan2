use super::File;
use std::path::{Path, PathBuf};
use nix::sys::statvfs::{statvfs};

#[allow(dead_code)]
//Take a mountpoint and check if it is ext4
pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    let dirinfo = statvfs(dir)?;
    print!("Result: {}", dirinfo.filesystem_id());
    Ok(dirinfo.filesystem_id() == 10)
}

pub fn scan(dir: PathBuf) -> Result<File, Box<dyn ::std::error::Error>> {
    todo!();
}
