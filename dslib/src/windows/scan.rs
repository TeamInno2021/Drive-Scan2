use super::drive::DriveInfo;
use super::winapi::{winapi_call, winapi_str};

use std::ptr;

use winapi::um::fileapi::{CreateFileW, GetVolumeNameForVolumeMountPointW, OPEN_EXISTING};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winnt::{FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ};

pub fn scan(drive: DriveInfo) -> Result<(), String> {
    let volume = winapi_call(
        50,
        |volume, size| unsafe {
            GetVolumeNameForVolumeMountPointW(winapi_str(drive.root), volume, size)
        },
        |code| code != 0,
    )
    .ok_or("unable to fetch target volume information")?;

    // Remove the any trailing backslashes from the volume info
    let volume = volume.to_str().unwrap().trim_end_matches("\\");

    // Open a handle to the target volume
    let volume_handle = unsafe {
        CreateFileW(
            winapi_str(volume),
            GENERIC_READ,
            FILE_SHARE_DELETE | FILE_SHARE_READ | FILE_SHARE_WRITE,
            ptr::null_mut(),
            OPEN_EXISTING,
            0,
            ptr::null_mut(),
        )
    };

    if volume_handle == INVALID_HANDLE_VALUE {
        return Err(format!(
            "unable to open volume {}, make sure program is running with elevated privileges",
            volume
        ));
    }

    // Cleanup
    unsafe { CloseHandle(volume_handle) };

    Ok(())
}
