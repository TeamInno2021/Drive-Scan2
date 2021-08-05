use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::OsStringExt;
use std::os::windows::prelude::OsStrExt;
use std::ptr;

use winapi::shared::ntdef::{LANG_NEUTRAL, MAKELANGID, SUBLANG_DEFAULT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::FormatMessageW;
use winapi::um::winbase::{FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS};

pub trait WinapiExt<T>
where
    Self: Sized,
{
    fn call<F, R, V>(size: usize, f: F, ok: V, truncate: bool) -> Result<Self, OsString>
    where
        F: FnOnce(usize, *mut T) -> R,
        V: FnOnce(R) -> bool;
}

impl WinapiExt<u8> for Vec<u8> {
    fn call<F, R, V>(size: usize, f: F, ok: V, truncate: bool) -> Result<Self, OsString>
    where
        F: FnOnce(usize, *mut u8) -> R,
        V: FnOnce(R) -> bool,
    {
        let mut buffer: Vec<u8> = Vec::with_capacity(size);

        let status = f(size, buffer.as_mut_ptr());

        if ok(status) {
            unsafe { buffer.set_len(size.into()) }

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

impl WinapiExt<u16> for Vec<u16> {
    fn call<F, R, V>(size: usize, f: F, ok: V, truncate: bool) -> Result<Self, OsString>
    where
        F: FnOnce(usize, *mut u16) -> R,
        V: FnOnce(R) -> bool,
    {
        let mut buffer: Vec<u16> = Vec::with_capacity(size);

        let status = f(size, buffer.as_mut_ptr());

        if ok(status) {
            unsafe { buffer.set_len(size.into()) }

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

impl WinapiExt<u8> for String {
    fn call<F, R, V>(size: usize, f: F, ok: V, truncate: bool) -> Result<Self, OsString>
    where
        F: FnOnce(usize, *mut u8) -> R,
        V: FnOnce(R) -> bool,
    {
        let raw: Vec<u8> = WinapiExt::call(size, f, ok, truncate)?;
        String::from_utf8(raw).map_err(|_| "attempted to convert non utf-8 buffer to utf-8".into())
    }
}

impl WinapiExt<u16> for OsString {
    fn call<F, R, V>(size: usize, f: F, ok: V, truncate: bool) -> Result<Self, OsString>
    where
        F: FnOnce(usize, *mut u16) -> R,
        V: FnOnce(R) -> bool,
    {
        let raw: Vec<u16> = WinapiExt::call(size, f, ok, truncate)?;
        Ok(OsString::from_wide(&raw))
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

/// Convert a rust string into a LPCWSTR
///
/// # Arguments
///
/// * `s` - The string to convert
///
/// # Safety
///
/// The caller must ensure that the input string outlives the returned pointer.
///
/// # Examples
///
/// ```
/// # use std::ffi::OsString;
/// let s = String::from("Hello, World!");
/// unsafe {
///     let ptr: *const u16 = to_lpcwstr(&s);
///     assert_eq!(*ptr, s.bytes().next().unwrap() as u16);
/// }
/// ```
pub unsafe fn to_lpcwstr<S>(s: S) -> *const u16
where
    S: AsRef<OsStr>,
{
    let mut bytes = OsStr::new(s.as_ref())
        .encode_wide()
        .chain(::std::iter::once(0))
        .collect::<Vec<u16>>();

    // Insert a null byte
    bytes.push(0);
    bytes.as_ptr()
}
