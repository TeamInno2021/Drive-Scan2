use super::OsError;
use std::ffi::OsString;
use std::path::Path;
use std::ptr;

use super::winapi::{winapi_call, winapi_str};
use winapi::shared::minwindef::MAX_PATH;
use winapi::um::fileapi::GetVolumeInformationW;

/// Identify the filesystem of the target device (e.g 'NTFS')
pub fn identify(dir: &Path) -> Result<OsString, OsError> {
    let root = dir.ancestors().last().unwrap(); // this unwrap is safe as .ancestors() always returns at least one value

    // Get the name of the partition system of the target device
    let system = winapi_call(
        MAX_PATH + 1,
        |system, size| unsafe {
            GetVolumeInformationW(
                winapi_str(root.to_str().unwrap()),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                system,
                size as u32,
            )
        },
        |code| code != 0,
    )?;

    Ok(system)
}
