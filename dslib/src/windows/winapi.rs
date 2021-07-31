//! winapi helper methods

use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr;

use winapi::shared::ntdef::{LANG_NEUTRAL, MAKELANGID, SUBLANG_DEFAULT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::{
    FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
};

/// Get the error message of the last winapi error using `GetLastError` and `FormatMessage`
pub fn get_last_error() -> OsString {
    let code = unsafe { GetLastError() };

    const BUFFER_SIZE: u32 = 16384; // 16kb
    let mut buffer: Vec<u16> = Vec::with_capacity(BUFFER_SIZE as usize);

    let size = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null(),
            code,
            MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT).into(),
            buffer.as_mut_ptr(),
            BUFFER_SIZE,
            ptr::null_mut(),
        )
    };

    unsafe { buffer.set_len(size as usize) }

    // If FormatMessage failed then inject our own error message
    if size == 0 {
        OsString::from("`FormatMessage` unexpectedly failed to fetch error status, this is likely a bug or a problem with your system")
    } else {
        OsString::from_wide(&buffer)
    }
}

/// A helper to make calling winapi functions from rust easier
///
/// # Arguments
/// - *size* - The buffer size, this is often passed directly to the function as well
/// - *func* - The closure which contains the function being called, this is passed a pointer to an lpstr to use as an output buffer. It is also passed the value passed to the `size` argument of this function.
/// - *success* - A closure which identifies the returned status code and checks whether the function succeeded, if it did not then `GetLastError` will be called and result in an `Err` message.
pub fn winapi_call<Size, Func, Success, Return>(
    size: Size,
    func: Func,
    success: Success,
) -> Result<OsString, OsString>
where
    Size: Into<usize> + Copy,
    Func: FnOnce(*mut u16, Size) -> Return,
    Success: FnOnce(Return) -> bool,
{
    let mut buffer: Vec<u16> = Vec::with_capacity(size.into());
    let code = func(buffer.as_mut_ptr(), size);

    if !success(code) {
        Err(get_last_error())
    } else {
        unsafe { buffer.set_len(size.into()) }

        // Remove any characters after the first null byte
        buffer.truncate(buffer.iter().position(|c| *c == 0).unwrap_or(buffer.len()));

        Ok(OsString::from_wide(&buffer))
    }
}

/// Convert a rust string into a win32 lpstr, appending a null byte
pub fn winapi_str<S>(s: S) -> *const u16
where
    S: AsRef<OsStr>,
{
    OsStr::new(s.as_ref())
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>()
        .as_ptr()
}
