// use std::ffi::OsString;
// use std::os::windows::ffi::OsStringExt;
// use std::ptr;

// use winapi::shared::ntdef::{LANG_NEUTRAL, MAKELANGID, SUBLANG_DEFAULT};
// use winapi::um::errhandlingapi::GetLastError;
// use winapi::um::winbase::FormatMessageW;
// use winapi::um::winbase::{FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS};

// pub struct WinapiResult<T>(Vec<T>);

// impl<T> From<WinapiResult<T>> for Vec<T> {
//     fn from(res: WinapiResult<T>) -> Self {
//         res.0
//     }
// }

// impl From<WinapiResult<u8>> for String {
//     fn from(res: WinapiResult<u8>) -> Self {
//         String::from_utf8_lossy(&res.0).to_string()
//     }
// }

// impl From<WinapiResult<u16>> for OsString {
//     fn from(res: WinapiResult<u16>) -> Self {
//         OsString::from_wide(&res.0)
//     }
// }

// // ------------------------------------------------------------

// pub unsafe fn winapi_call<Size, Ty, WinapiFn, IsOk, Status>(
//     size: Size,
//     call: WinapiFn,
//     ok: IsOk,
// ) -> Result<WinapiResult<Ty>, OsString>
// where
//     Ty: PartialEq<usize>,
//     Size: Copy + Into<usize>,
//     WinapiFn: FnOnce(*mut Ty, Size) -> Status,
//     IsOk: FnOnce(Status) -> bool,
// {
//     let mut buffer: Vec<Ty> = Vec::with_capacity(size.into());

//     let status = call(buffer.as_mut_ptr(), size);
//     if ok(status) {
//         // Remove any data after the first null byte
//         buffer.truncate(buffer.iter().position(|c| c == &0).unwrap_or(buffer.len()));

//         Ok(WinapiResult(buffer))
//     } else {
//         Err(last_error())
//     }
// }

// /// Get the error message of the last winapi error using `GetLastError` and `FormatMessage`
// pub fn last_error() -> OsString {
//     let code = unsafe { GetLastError() };

//     const BUFFER_SIZE: u32 = 16384; // 16kb (just in case)
//     let mut buffer: Vec<u16> = Vec::with_capacity(BUFFER_SIZE as usize);

//     let size = unsafe {
//         FormatMessageW(
//             FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
//             ptr::null(),
//             code,
//             MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT).into(),
//             buffer.as_mut_ptr(),
//             BUFFER_SIZE,
//             ptr::null_mut(),
//         )
//     };

//     unsafe { buffer.set_len(size as usize) }

//     // If FormatMessage failed then inject our own error message
//     if size == 0 {
//         OsString::from("`FormatMessage` unexpectedly failed to fetch error status, this is likely a bug or a problem with your system")
//     } else {
//         OsString::from_wide(&buffer)
//     }
// }
