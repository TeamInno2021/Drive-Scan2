use super::OsError;
use std::ffi::OsString;
use std::path::Path;
use std::ptr;

use super::winapi::{to_utf16_ptr, WinapiCall};
use winapi::shared::minwindef::MAX_PATH;
use winapi::um::fileapi::GetVolumeInformationW;

/// Identify the filesystem of the target device (e.g 'NTFS')
pub fn identify(dir: &Path) -> Result<OsString, OsError> {
    let root = dir.ancestors().last().unwrap(); // this unwrap is safe as .ancestors() always returns at least one value

    // Get the name of the partition system of the target device
    let system = OsString::winapi_call(
        MAX_PATH + 1,
        |size, system| unsafe {
            GetVolumeInformationW(
                to_utf16_ptr(
                    root.to_str()
                        .expect("unexpected invalid utf-8 in volume name"),
                ),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                system,
                size as u32,
            )
        },
        |status| status != 0,
        true,
    )?;

    Ok(system)
}
