use std::ffi::OsString;

#[cfg(windows)]
use std::os::windows::ffi::OsStringExt;

// Buffer type (e.g u8 or u16)
// Buffer size (usize)
// Function to call (takes buffer pointer + buffer size)
// Check return code if it has one with second func

// Return should be an object which can be either converted into a wide string or a byte array

pub struct WinapiResult<T>(Vec<T>);

impl<T> From<WinapiResult<T>> for Vec<T> {
    fn from(res: WinapiResult<T>) -> Self {
        res.0
    }
}

impl From<WinapiResult<u8>> for String {
    fn from(res: WinapiResult<u8>) -> Self {
        String::from_utf8_lossy(&res.0).to_string()
    }
}

#[cfg(windows)]
impl From<WinapiResult<u16>> for OsString {
    fn from(res: WinapiResult<u16>) -> Self {
        OsString::from_wide(&res.0)
    }
}

pub unsafe fn winapi_call<Ty, Size>(size: Size) -> Result<WinapiResult<Ty>, OsString>
where
    Ty: Copy + Into<usize>,
    Size: Into<usize>,
{
    let size = size.into();

    let mut buffer: Vec<Ty> = Vec::with_capacity(size);

    // todo call func and check return code

    // Remove any data after the first null byte
    buffer.truncate(
        buffer
            .iter()
            .position(|c| (*c).into() == 0)
            .unwrap_or(buffer.len()),
    );

    Ok(WinapiResult(buffer))
}
/*
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
*/
