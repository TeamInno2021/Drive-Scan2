//! winapi helper methods

use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr;

use winapi::shared::ntdef::{LANG_NEUTRAL, MAKELANGID, SUBLANG_DEFAULT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::{
    FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
};

/// Convert an error code into an error message using the `FormatMessage` method
fn extract_error(code: u32) -> Option<OsString> {
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

    if size == 0 {
        // If zero bytes were stored then the function failed
        None
    } else {
        unsafe { buffer.set_len(size as usize) }
        Some(OsString::from_wide(&buffer))
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
        // Extract the actual error code as a concrete type
        let code = unsafe { GetLastError() };

        Err(extract_error(code)
            .unwrap_or_else(|| format!("unable to locate error message for code: {}", code).into()))
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
    S: AsRef<str>,
{
    OsStr::new(s.as_ref())
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>()
        .as_ptr()
}
