use super::{data, File};
use std::path::PathBuf;

use winapi::um::fileapi::{FindClose, FindFirstFileA, FindNextFileA};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::WIN32_FIND_DATAA;
use winapi::um::winnt::FILE_ATTRIBUTE_DIRECTORY;

/// An alternate scanner which functions on any drive and does not need elevated permissions.
///
/// Note that this is several times slower than the primary scanner, recursion!
/// (also does not support the full utf-16 windows paths)
pub fn scan(dir: PathBuf) {
    let file = unsafe { _scan(dir) };
    info!("Scanned {} bytes", file.size);
    data::store(file);
}

unsafe fn _scan(dir: PathBuf) -> File {
    trace!("Scanning {:?}", dir);
    let mut children: Vec<File> = Vec::new();
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
        return File {
            path: dir,
            size: 0,
            children: Some(Vec::new()),
        };
    }

    loop {
        let raw = ffd.cFileName.as_ptr();
        let len = (0..).take_while(|&i| *raw.offset(i) != 0).count(); // find the first null byte from the start of the name
        let name =
            String::from_utf8_lossy(std::slice::from_raw_parts(raw as *const u8, len)).to_string();
        let path = dir.join(&name);

        if name != "." && name != ".." {
            // If directory
            if ffd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY != 0 {
                trace!("{:?} : directory", name);
                let file = _scan(path);
                size += file.size;
                children.push(file);
            } else {
                let _size = ffd.nFileSizeHigh as usize * (u32::MAX as usize + 1)
                    + ffd.nFileSizeLow as usize;
                trace!("{:?} : file | total: {} + {} bytes", name, size, _size);
                let file = File {
                    path,
                    size: _size,
                    children: None,
                };
                size += _size;
                children.push(file);
            }
        }

        // If we are out of files to read
        if FindNextFileA(handle, &mut ffd) == 0 {
            break;
        }
    }

    FindClose(handle);

    File {
        path: dir,
        size,
        children: Some(children),
    }
}
