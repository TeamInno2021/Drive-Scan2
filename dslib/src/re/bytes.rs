use std::marker::PhantomData;
use std::{fmt, slice};

pub trait BytesInner<T> {}
pub type BytesInnerVec<T> = Vec<T>;
pub type BytesInnerMutPtr<T> = *mut T;
pub type BytesInnerConstPtr<T> = *const T;

impl<T> BytesInner<T> for BytesInnerVec<T> {}
impl<T> BytesInner<T> for BytesInnerMutPtr<T> {}
impl<T> BytesInner<T> for BytesInnerConstPtr<T> {}

/// An abstraction over a contiguous slice of memory,
/// which is not necessarily made up of single bytes.
pub struct Bytes<'a, T, I>
where
    T: 'a,
    I: BytesInner<T>,
{
    inner: I,
    phantom: PhantomData<&'a T>,
}

// impl<'a, T, I> Bytes<'a, T, I>
// where
//     T: 'a,
//     I: BytesInner<T>,
// {

// }

// -------------------- BytesInnerVec ----------------------

impl<'a, T: 'a> Bytes<'a, T, BytesInnerVec<T>> {
    pub fn new() -> Self {
        Vec::new().into()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity).into()
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    pub fn position<P>(&self, mut predicate: P) -> Option<usize>
    where
        P: FnMut(&T) -> bool,
    {
        self.inner.iter().position(|t| predicate(t));

        None
    }

    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.inner.as_mut_ptr()
    }
}

impl<'a> Bytes<'a, u8, BytesInnerVec<u8>> {
    pub fn into_utf8(self) -> Result<String, ::std::string::FromUtf8Error> {
        String::from_utf8(self.inner)
    }
}

#[cfg(windows)]
impl<'a> Bytes<'a, u16, BytesInnerVec<u16>> {
    pub fn to_utf16(&self) -> ::std::ffi::OsString {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;

        OsString::from_wide(&self.inner)
    }
}

impl<'a, T: 'a> From<Vec<T>> for Bytes<'a, T, BytesInnerVec<T>> {
    fn from(vec: Vec<T>) -> Self {
        Bytes {
            inner: vec,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: 'a> Default for Bytes<'a, T, BytesInnerVec<T>> {
    fn default() -> Self {
        Vec::default().into()
    }
}

impl<'a, T: fmt::Debug + 'a> fmt::Debug for Bytes<'a, T, BytesInnerVec<T>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl<'a, T: Clone + 'a> Clone for Bytes<'a, T, BytesInnerVec<T>> {
    fn clone(&self) -> Self {
        Bytes {
            inner: self.inner.clone(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T: PartialEq + 'a> PartialEq for Bytes<'a, T, BytesInnerVec<T>> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn ne(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

impl<'a, T: Eq + 'a> Eq for Bytes<'a, T, BytesInnerVec<T>> {}

// -------------------- BytesInnerMutPtr ----------------------

impl<'a, T: Clone + 'a> Bytes<'a, T, BytesInnerMutPtr<T>> {
    pub unsafe fn as_vec(&self, len: usize) -> Bytes<'a, T, BytesInnerVec<T>> {
        let raw = slice::from_raw_parts(self.inner, len);
        Bytes::from(raw.to_vec())
    }
}

impl<'a, T: 'a> From<*mut T> for Bytes<'a, T, BytesInnerMutPtr<T>> {
    fn from(ptr: *mut T) -> Self {
        Bytes {
            inner: ptr,
            phantom: PhantomData,
        }
    }
}

// -------------------- BytesInnerConstPtr ----------------------

impl<'a, T: Clone + 'a> Bytes<'a, T, BytesInnerConstPtr<T>> {
    pub unsafe fn as_vec(&self, len: usize) -> Bytes<'a, T, BytesInnerVec<T>> {
        let raw = slice::from_raw_parts(self.inner, len);
        Bytes::from(raw.to_vec())
    }
}

impl<'a, T: 'a> From<*const T> for Bytes<'a, T, BytesInnerConstPtr<T>> {
    fn from(ptr: *const T) -> Self {
        Bytes {
            inner: ptr,
            phantom: PhantomData,
        }
    }
}
