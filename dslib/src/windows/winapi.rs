//! win32api helper methods

use std::convert::TryInto;
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};

// use winapi::shared::ntdef::LANG_USER_DEFAULT;
// use winapi::um::winbase::{
//     FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
// };

/// The return value of a winapi_call closure
pub trait WinapiReturn {
    const IDENT: WinapiReturnType;

    fn downcast(self) -> Option<i32>
    where
        Self: Sized,
    {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum WinapiReturnType {
    unit,
    i32,
}

impl WinapiReturn for () {
    const IDENT: WinapiReturnType = WinapiReturnType::unit;
}

impl WinapiReturn for i32 {
    const IDENT: WinapiReturnType = WinapiReturnType::i32;

    fn downcast(self) -> Option<i32> {
        Some(self)
    }
}

impl WinapiReturn for u32 {
    const IDENT: WinapiReturnType = WinapiReturnType::i32;

    fn downcast(self) -> Option<i32> {
        Some(self.try_into().unwrap())
    }
}

/// Convert a rust str to a win32 lpstr, appending a null byte
pub fn to_wstring(val: &str) -> Vec<u16> {
    OsStr::new(val)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/// Call arbitary code with an automatic buffer for handling output values from wide winapi calls
///
/// # Arguments
/// - *size* - The buffer size, this is often passed directly to the function as well
/// - *f* - The closure which contains the function being called, this is passed a pointer to an lpstr to use as an output buffer. It is also passed the value passed to the `size` argument of this function.
/// - *sucess* - A closure which identifies the returned status code and checks whether the function succeeded, if it did not then `GetLastError` will be called and result in an error.
///              Note that for closures which do not return a status code, the `success` method will be ignored.
// todo make function return `Result<OsString, Error>` and handle the lookup with `FormatMessage`
pub fn winapi_call<R, F, Fs>(size: u32, f: F, success: Fs) -> Option<OsString>
where
    R: WinapiReturn,
    F: FnOnce(*mut u16, u32) -> R,
    Fs: FnOnce(i32) -> bool,
{
    let mut str: Vec<u16> = Vec::with_capacity(size as usize);
    let res = f(str.as_mut_ptr(), size);

    match R::IDENT {
        WinapiReturnType::unit => (),
        WinapiReturnType::i32 => {
            let status = res.downcast().unwrap();
            if !success(status) {
                return None;

                // let mut message = Vec::with_capacity(1024);
                // let len = unsafe {
                //     FormatMessageW(
                //         FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
                //         ptr::null(),
                //         status as u32,
                //         LANG_USER_DEFAULT as u32,
                //         message.as_mut_ptr(),
                //         1024,
                //         ptr::null_mut(),
                //     )
                // };

                // if len == 0 {
                //     // if the FormatMessage function returns an error this could indicate a deeper problem
                //     // abort the program to prevent a stack overflow or other undefined behaviour
                //     panic!("unexpected error from winapi `FormatMessage` routine, aborting...")
                // }

                // unsafe { str.set_len(len as usize) };
                // let message = OsString::from_wide(&str);
                // // println!("{:#?}", message);

                // return Err(message.into());
            }
        }
    };

    unsafe { str.set_len(size as usize) };

    // remove any characters after the first null byte
    str.truncate(str.iter().position(|c| *c == 0).unwrap_or(str.len()));

    Some(OsString::from_wide(&str))
}

/// Convert a rust string into a wide string pointer the winapi expects
pub fn winapi_str<S>(s: S) -> *const u16
where
    S: AsRef<str>,
{
    to_wstring(&s.as_ref()).as_ptr()
}
