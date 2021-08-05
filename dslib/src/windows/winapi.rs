use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::OsStringExt;
use std::os::windows::prelude::OsStrExt;
use std::ptr;

use winapi::shared::ntdef::{LANG_NEUTRAL, MAKELANGID, SUBLANG_DEFAULT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::FormatMessageW;
use winapi::um::winbase::{FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS};

pub trait WinapiCall<T>
where
    Self: Sized,
{
    fn winapi_call<F, R, V>(size: usize, f: F, ok: V, truncate: bool) -> Result<Self, OsString>
    where
        F: FnOnce(usize, *mut T) -> R,
        V: FnOnce(R) -> bool;
}

impl WinapiCall<u8> for Vec<u8> {
    fn winapi_call<F, R, V>(size: usize, f: F, ok: V, truncate: bool) -> Result<Self, OsString>
    where
        F: FnOnce(usize, *mut u8) -> R,
        V: FnOnce(R) -> bool,
    {
        let mut buffer: Vec<u8> = Vec::with_capacity(size);

        let status = f(size, buffer.as_mut_ptr());

        if ok(status) {
            if truncate {
                // Remove any data after the first null byte
                buffer.truncate(buffer.iter().position(|c| c == &0).unwrap_or(buffer.len()));
            }

            Ok(buffer)
        } else {
            Err(get_last_error())
        }
    }
}

impl WinapiCall<u8> for String {
    fn winapi_call<F, R, V>(size: usize, f: F, ok: V, truncate: bool) -> Result<Self, OsString>
    where
        F: FnOnce(usize, *mut u8) -> R,
        V: FnOnce(R) -> bool,
    {
        let raw = Vec::<u8>::winapi_call(size, f, ok, truncate)?;
        String::from_utf8(raw).map_err(|_| "attempted to convert non utf-8 buffer to utf-8".into())
    }
}

impl WinapiCall<u16> for OsString {
    fn winapi_call<F, R, V>(size: usize, f: F, ok: V, truncate: bool) -> Result<Self, OsString>
    where
        F: FnOnce(usize, *mut u16) -> R,
        V: FnOnce(R) -> bool,
    {
        let mut buffer: Vec<u16> = Vec::with_capacity(size);

        let status = f(size, buffer.as_mut_ptr());

        if ok(status) {
            if truncate {
                // Remove any data after the first null byte
                buffer.truncate(buffer.iter().position(|c| c == &0).unwrap_or(buffer.len()));
            }

            Ok(OsString::from_wide(&buffer))
        } else {
            Err(get_last_error())
        }
    }
}

/// Get the error message of the last winapi error using `GetLastError` and `FormatMessage`
pub fn get_last_error() -> OsString {
    let code = unsafe { GetLastError() };

    const BUFFER_SIZE: u32 = 16384; // 16kb (just in case)
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

/// Convert a rust string into a pointer correctly encoded utf-16 string (LPWSTR)
pub fn to_utf16_ptr<S>(s: S) -> *const u16
where
    S: AsRef<OsStr>,
{
    OsStr::new(s.as_ref())
        .encode_wide()
        .chain(::std::iter::once(0))
        .collect::<Vec<u16>>()
        .as_ptr()
}
