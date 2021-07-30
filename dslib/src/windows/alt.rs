use super::OsError;

use std::ffi::OsString;
use std::os::windows::prelude::{OsStrExt, OsStringExt};
use std::path::PathBuf;
use std::sync::atomic::{self, AtomicUsize};

use winapi::shared::minwindef::MAX_PATH;
use winapi::um::fileapi::{FindClose, FindFirstFileW, FindNextFileW};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::{LPWIN32_FIND_DATAW, WIN32_FIND_DATAW};
use winapi::um::winnt::FILE_ATTRIBUTE_DIRECTORY;

static DEPTH: AtomicUsize = AtomicUsize::new(0);

/// An alternate scanner which functions on any drive and does not need elevated permissions.
///
/// Note that this is several times slower than the primary scanner
pub fn scan(mut dir: PathBuf) -> Result<(), OsError> {
    DEPTH.store(0, atomic::Ordering::SeqCst);

    dir.push("*\0");

    let mut dir: Vec<u16> = dir.as_os_str().encode_wide().collect();
    dir.resize(MAX_PATH, '\0' as u16);

    let mut ffd = WIN32_FIND_DATAW::default();

    _scan(dir.as_ptr(), &mut ffd);

    //     let mut name = ffd.cFileName.to_vec();
    //     name.reverse();
    //     let length = ffd
    //         .cFileName
    //         .to_vec()
    //         .iter()
    //         .position(|c| *c == '\0' as u16)
    //         .map(|l| l - 1)
    //         .unwrap_or_else(|| name.len());
    //     name.reverse();
    //     name.truncate(length + 1);
    //     let name = OsString::from_wide(&name);

    //         let low = ffd.nFileSizeHigh.to_le_bytes();
    //         let high = ffd.nFileSizeLow.to_le_bytes();
    //         let size = u64::from_le_bytes([
    //             low[0], low[1], low[2], low[3], high[0], high[1], high[2], high[3],
    //         ]);

    Ok(())
}

fn _scan(name: *const u16, ffd: LPWIN32_FIND_DATAW) {
    let current = DEPTH.fetch_add(1, atomic::Ordering::SeqCst);
    trace!("depth: {}", current);

    let handle = unsafe { FindFirstFileW(name, ffd) };

    if handle == INVALID_HANDLE_VALUE {
        panic!("unable to find first file");
    }

    loop {
        if unsafe { (*ffd).dwFileAttributes } & FILE_ATTRIBUTE_DIRECTORY != 0 {
            _scan(unsafe { (*ffd).cFileName.as_ptr() }, ffd);
        } else {
            // todo file
            trace!("got file");
        }

        if unsafe { FindNextFileW(handle, ffd) } == 0 {
            break;
        }
    }

    unsafe { FindClose(handle) };
}
