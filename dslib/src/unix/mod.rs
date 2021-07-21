use super::File;
use std::path::{Path, PathBuf};
use nix::sys::statfs::{statfs};

//Take a mountpoint and check if it is either ext or ntfs othewrise return false
pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    // List of magic numbers for filesystem ids
                        //ext,    ntfs,    
    let supportedtypes = [0xef53, 0x534654e];
    trace!("Attempting to query folder: \"{}\" for partition type...", dir.to_str().unwrap());
    let dirinfo = statfs(dir)?;
    match dirinfo.filesystem_type().0 {
        0xef53    => trace!("Folder located on an ext parititon - Proceeding with Scan"),
        0x534654e => trace!("Folder located on an ntfs parititon - Proceeding with Scan"),
                _ => trace!("Folder located on unsupported partition type - Falling back to Universal Scanner (N.Y.I)")
    }
    Ok(supportedtypes.contains(&dirinfo.filesystem_type().0))
}

pub fn scan(dir: PathBuf) -> Result<File, Box<dyn ::std::error::Error>> {
    Ok(File::File { path: dir, size: 0 })
}
