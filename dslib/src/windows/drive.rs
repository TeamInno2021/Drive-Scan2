use super::OsError;
use std::ffi::c_void;
use std::path::PathBuf;
use std::ptr;

use super::winapi::{get_last_error, winapi_call, winapi_str};
use winapi::um::fileapi::{CreateFileW, GetVolumeNameForVolumeMountPointW, OPEN_EXISTING};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winnt::{FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ};

#[derive(Debug)]
pub struct DriveInfo {
    /// The root path of the drive
    pub root: String,
    /// The drive letter
    pub letter: char,
    /// The name of the drive volume
    pub volume: String,
    /// A handle to the target volume
    pub volume_handle: *mut c_void,
}

impl DriveInfo {
    pub fn parse(path: PathBuf) -> Result<Self, OsError> {
        let root = path
            .ancestors()
            .last()
            .unwrap()
            .to_string_lossy()
            .to_string(); // this unwrap is safe as .ancestors() always returns at least one value

        let letter = path
            .components()
            .next()
            .unwrap()
            .as_os_str()
            .to_string_lossy()
            .chars()
            .next()
            .unwrap();

        let volume = winapi_call(
            50 as usize,
            |volume, size| unsafe {
                GetVolumeNameForVolumeMountPointW(winapi_str(&root), volume, size as u32)
            },
            |code| code != 0,
        )?
        .to_str()
        .unwrap()
        .trim_end_matches("\\") // remove trailing backslashes
        .to_string();

        let volume_handle = unsafe {
            CreateFileW(
                winapi_str(&volume),
                GENERIC_READ,
                FILE_SHARE_DELETE | FILE_SHARE_READ | FILE_SHARE_WRITE,
                ptr::null_mut(),
                OPEN_EXISTING,
                0,
                ptr::null_mut(),
            )
        };

        if volume_handle == INVALID_HANDLE_VALUE {
            return Err(get_last_error().into());
        }

        Ok(DriveInfo {
            root,
            letter,
            volume,
            volume_handle,
        })
    }
}

impl Drop for DriveInfo {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.volume_handle) };
    }
}
