use super::OsError;

use std::os::windows::prelude::OsStrExt;
use std::path::PathBuf;

use winapi::um::fileapi::FindFirstFileW;
use winapi::um::minwinbase::WIN32_FIND_DATAW;

/// An alternate scanner which functions on any drive and does not need elevated permissions.
///
/// Note that this is several times slower than the primary scanner
pub fn scan(dir: PathBuf) -> Result<(), OsError> {
    let mut ffd = WIN32_FIND_DATAW::default();
    let dir: Vec<u16> = dir.into_os_string().encode_wide().collect();
    let _handle = unsafe { FindFirstFileW(dir.as_ptr(), &mut ffd) };

    // loop {}

    Ok(())
}
