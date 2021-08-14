// use super::File;
// use std::ffi::OsString;
// use std::os::windows::prelude::OsStringExt;
// use std::path::PathBuf;

// use super::winapi::winapi_str;
// use winapi::um::fileapi::{FindClose, FindFirstFileW, FindNextFileW};
// use winapi::um::handleapi::INVALID_HANDLE_VALUE;
// use winapi::um::minwinbase::WIN32_FIND_DATAW;
// use winapi::um::winnt::FILE_ATTRIBUTE_DIRECTORY;

// lazy_static::lazy_static! {
//     pub static ref DATA: Option<File> = None;
// }

// /// An alternate scanner which functions on any drive and does not need elevated permissions.
// ///
// /// Note that this is several times slower than the primary scanner, recursion!
// pub fn scan(dir: PathBuf) {
//     _scan(dir);
// }

// fn _scan(dir: PathBuf) {
//     trace!("Scanning {:?}", dir);

//     let mut ffd = WIN32_FIND_DATAW::default();
//     let handle = unsafe { FindFirstFileW(winapi_str(dir.join("*\0")), &mut ffd) };

//     if handle == INVALID_HANDLE_VALUE {
//         warn!("unable to find first file in directory, skipping...");
//         return;
//     }

//     loop {
//         let name_raw = ffd.cFileName.as_ptr();
//         let len = unsafe { (0..).take_while(|&i| *name_raw.offset(i) != 0).count() }; // find the first null byte from the start of the name
//         let name = OsString::from_wide(unsafe { std::slice::from_raw_parts(name_raw, len) });

//         if ffd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY != 0 {
//             if name != "." && name != ".." {
//                 // todo record directory
//                 _scan(dir.join(name));
//             }
//         } else {
//             let size =
//                 ffd.nFileSizeHigh as usize * (u32::MAX as usize + 1) + ffd.nFileSizeLow as usize;
//             // todo record file
//             trace!("got file {:?} with size {} bytes", name, size);
//         }

//         if unsafe { FindNextFileW(handle, &mut ffd) } == 0 {
//             break;
//         }
//     }

//     unsafe { FindClose(handle) };
// }
