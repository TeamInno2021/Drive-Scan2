use super::OsError;

use std::ffi::OsString;
use std::os::windows::prelude::OsStringExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{self, AtomicUsize};

use super::winapi::winapi_str;
use winapi::um::fileapi::{FindClose, FindFirstFileW, FindNextFileW};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::{LPWIN32_FIND_DATAW, WIN32_FIND_DATAW};
use winapi::um::winnt::FILE_ATTRIBUTE_DIRECTORY;

static DEPTH: AtomicUsize = AtomicUsize::new(0);

/// An alternate scanner which functions on any drive and does not need elevated permissions.
///
/// Note that this is several times slower than the primary scanner
pub fn scan(dir: PathBuf) -> Result<(), OsError> {
    DEPTH.store(0, atomic::Ordering::Relaxed);

    let mut ffd = WIN32_FIND_DATAW::default();

    _scan(&dir, &mut ffd);

    assert_eq!(DEPTH.fetch_sub(1, atomic::Ordering::Relaxed), 0);

    Ok(())
}

fn _scan(path: &Path, ffd: LPWIN32_FIND_DATAW) {
    let current = DEPTH.fetch_add(1, atomic::Ordering::Relaxed);
    trace!("depth: {} | {:?}", current, path);

    let handle = unsafe { FindFirstFileW(winapi_str(path.join("*\0")), ffd) };

    if handle == INVALID_HANDLE_VALUE {
        DEPTH.fetch_sub(1, atomic::Ordering::Relaxed);
        warn!("unable to find first file in directory, skipping...");
        return;
    }

    loop {
        let name_raw = unsafe { (*ffd).cFileName.as_ptr() };
        let len = unsafe { (0..).take_while(|&i| *name_raw.offset(i) != 0).count() };
        let name = OsString::from_wide(unsafe { std::slice::from_raw_parts(name_raw, len) });

        if unsafe { (*ffd).dwFileAttributes } & FILE_ATTRIBUTE_DIRECTORY != 0 {
            if name != "." && name != ".." {
                // todo record directory
                _scan(&path.join(name), ffd);
            }
        } else {
            let size = unsafe {
                (*ffd).nFileSizeHigh as usize * (u32::MAX as usize + 1)
                    + (*ffd).nFileSizeLow as usize
            };
            // todo record file
            trace!("got file {:?} with size {} bytes", name, size);
        }

        if unsafe { FindNextFileW(handle, ffd) } == 0 {
            break;
        }
    }

    DEPTH.fetch_sub(1, atomic::Ordering::Relaxed);
    unsafe { FindClose(handle) };
}
