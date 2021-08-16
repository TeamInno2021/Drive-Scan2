use super::{File, OsError};
use std::path::PathBuf;

use winapi::um::fileapi::{FindClose, FindFirstFileA, FindNextFileA};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::WIN32_FIND_DATAA;
use winapi::um::winnt::FILE_ATTRIBUTE_DIRECTORY;

lazy_static::lazy_static! {
    pub static ref DATA: Option<File> = None;
}

/// An alternate scanner which functions on any drive and does not need elevated permissions.
///
/// Note that this is several times slower than the primary scanner, recursion!
pub fn scan(dir: PathBuf) {
    let size = unsafe { _scan(dir) };
    println!("Total: {} bytes", size);
}

unsafe fn _scan(dir: PathBuf) -> usize {
    trace!("Scanning {:?}", dir);
    let mut size = 0;

    let mut ffd = WIN32_FIND_DATAA::default();
    let handle = FindFirstFileA(
        dir.join("*\0").as_path().to_string_lossy().as_ptr() as *const i8,
        &mut ffd,
    );

    if handle == INVALID_HANDLE_VALUE {
        warn!(
            "unable to find first file in directory ({:?}), skipping...",
            dir
        );
        return 0;
    }

    loop {
        let raw = ffd.cFileName.as_ptr();
        let len = (0..).take_while(|&i| *raw.offset(i) != 0).count(); // find the first null byte from the start of the name
        let name =
            String::from_utf8_lossy(std::slice::from_raw_parts(raw as *const u8, len)).to_string();

        if name != "." && name != ".." {
            // If directory
            if ffd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY != 0 {
                trace!("{:?} : directory", name);
                size += _scan(dir.join(name));
            } else {
                let _size = ffd.nFileSizeHigh as usize * (u32::MAX as usize + 1)
                    + ffd.nFileSizeLow as usize;
                trace!("{:?} : file | total: {} + {} bytes", name, size, _size);
                size += _size;
            }
        }

        // If we are out of files to read
        if FindNextFileA(handle, &mut ffd) == 0 {
            break;
        }
    }

    FindClose(handle);

    size
}
