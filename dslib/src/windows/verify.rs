use std::path::Path;
use std::ptr;

use super::winapi::{winapi_call, winapi_str};
use super::Error;
use winapi::shared::minwindef::MAX_PATH;
use winapi::um::fileapi::GetVolumeInformationW;

/// Ensure we are on an NTFS drive
pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    let root = dir.ancestors().last().unwrap(); // this unwrap is safe as .ancestors() always returns at least one value

    // Get the name of the partition system of the target device
    let system = winapi_call(
        MAX_PATH + 1,
        |system| unsafe {
            GetVolumeInformationW(
                winapi_str(root.to_str().unwrap()),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                system,
                (MAX_PATH + 1) as u32,
            )
        },
        |code| code != 0,
    )
    .ok_or("unable to fetch volume information")?;

    info!("Detected partition type {:?} for device {:?}", system, root);

    // note that the current system only supports NTFS drives
    Ok(system == "NTFS")
}
