use std::{error, fmt, slice};

pub struct FromBytesUntilNulError;

impl Clone for FromBytesUntilNulError {
    fn clone(&self) -> Self {
        Self
    }
}

impl fmt::Debug for FromBytesUntilNulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FromBytesUntilNulError()")
    }
}

impl fmt::Display for FromBytesUntilNulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Data provided does not contain a nul")
    }
}

impl error::Error for FromBytesUntilNulError {}

pub enum FromBytesWithNulError {
    InteriorNul { position: usize },
    NotNulTerminated,
}

impl Clone for FromBytesWithNulError {
    fn clone(&self) -> Self {
        match self {
            Self::InteriorNul { position } => Self::InteriorNul {
                position: *position,
            },

            Self::NotNulTerminated => Self::NotNulTerminated,
        }
    }
}

impl fmt::Debug for FromBytesWithNulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InteriorNul { position } => {
                f.write_fmt(format_args!("InteriorNul {{ position: {:?} }}", position))
            }

            Self::NotNulTerminated => f.write_str("NotNulTerminated"),
        }
    }
}

impl fmt::Display for FromBytesWithNulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InteriorNul { position } => f.write_fmt(format_args!(
                "Data provided contains an interior nul byte at position: {}",
                position
            )),

            Self::NotNulTerminated => f.write_str("Data provided is not nul terminated"),
        }
    }
}

impl error::Error for FromBytesWithNulError {}

pub struct WCStr([u16]);

impl WCStr {
    pub const fn from_wide_until_nul(data: &[u16]) -> Result<&Self, FromBytesUntilNulError> {
        match memchr(0, data) {
            Some(idx) => unsafe {
                let slice = slice::from_raw_parts(data.as_ptr(), idx + 1);
                Ok(Self::from_wide_with_nul_unchecked(slice))
            },

            None => Err(FromBytesUntilNulError),
        }
    }

    pub const fn from_wide_with_nul(data: &[u16]) -> Result<&Self, FromBytesWithNulError> {
        match memchr(0, data) {
            Some(idx) if idx + 1 == data.len() => unsafe {
                Ok(Self::from_wide_with_nul_unchecked(data))
            },

            Some(idx) => Err(FromBytesWithNulError::InteriorNul { position: idx }),
            None => Err(FromBytesWithNulError::NotNulTerminated),
        }
    }

    #[inline]
    pub const unsafe fn from_wide_with_nul_unchecked(data: &[u16]) -> &Self {
        &*(data as *const [u16] as *const WCStr)
    }

    #[inline]
    pub const unsafe fn from_ptr<'ws>(data: *const u16) -> &'ws Self {
        let len = strlen(data);
        Self::from_wide_with_nul_unchecked(slice::from_raw_parts(data.cast(), len + 1))
    }

    #[inline]
    pub const fn to_wide(&self) -> &[u16] {
        unsafe { slice::from_raw_parts(self.0.as_ptr(), self.0.len() - 1) }
    }

    #[inline]
    pub const fn to_wide_with_nul(&self) -> &[u16] {
        unsafe { slice::from_raw_parts(self.0.as_ptr(), self.0.len()) }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }
}

impl AsRef<WCStr> for WCStr {
    fn as_ref(&self) -> &WCStr {
        self
    }
}

impl fmt::Debug for &WCStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() != 0 {
            f.write_str("[")?;
            for idx in 0..self.0.len() {
                f.write_fmt(format_args!(" {:02X} ", self.0[idx]))?;
            }
            f.write_str("]")?;

            Ok(())
        } else {
            f.write_str("[]")
        }
    }
}

const fn memchr(x: u16, data: &[u16]) -> Option<usize> {
    let mut idx = 0;

    loop {
        if idx >= data.len() {
            return None;
        }

        if data[idx] == x {
            return Some(idx);
        }

        idx += 1
    }
}

const unsafe fn strlen(data: *const u16) -> usize {
    let mut len = 0;

    loop {
        if *data.add(len) == 0 {
            return len;
        }

        len += 1
    }
}

#[cfg(test)]
mod unittest {
    use crate::ffi::WCStr;

    #[test]
    pub fn wcstr_wraps_from_wide_until_nul_when_data_contains_nul_test() {
        let data = &[1u16, 1, 1, 0, 1, 1];

        let wcstr =
            WCStr::from_wide_until_nul(data).expect("Data with nul was not wrapped with WCStr");

        assert_eq!(&[1u16, 1, 1, 0], &wcstr.0);
    }

    #[test]
    pub fn wcstr_fails_to_wrap_from_wide_until_nul_when_data_does_not_contains_nul_test() {
        let data = &[1u16, 1, 1, 1, 1, 1];

        let _ =
            WCStr::from_wide_until_nul(data).expect_err("Data without nul was wrapped with WCStr");
    }

    #[test]
    pub fn wcstr_wraps_from_wide_with_nul_when_data_contains_last_nul_test() {
        let data = &[1u16, 1, 1, 1, 1, 0];

        let wcstr =
            WCStr::from_wide_with_nul(data).expect("Data with last nul was not wrapped with WCStr");

        assert_eq!(&[1u16, 1, 1, 1, 1, 0], &wcstr.0);
    }

    #[test]
    pub fn wcstr_fails_to_wrap_from_wide_with_nul_when_data_does_not_contains_nul_test() {
        let data = &[1u16, 1, 1, 1, 1, 1];

        let _ =
            WCStr::from_wide_with_nul(data).expect_err("Data without nul was wrapped with WCStr");
    }

    #[test]
    pub fn wcstr_fails_to_wrap_from_wide_with_nul_when_data_contains_interior_nul_test() {
        let data = &[1u16, 1, 1, 0, 1, 1];

        let _ = WCStr::from_wide_with_nul(data)
            .expect_err("Data with interior nul was wrapped with WCStr");
    }

    #[test]
    pub fn wcstr_wraps_from_ptr_when_data_contains_nul_test() {
        let data = &[1u16, 1, 1, 1, 1, 0];

        let wcstr = unsafe { WCStr::from_ptr(data as *const u16) };

        assert_eq!(&[1u16, 1, 1, 1, 1, 0], &wcstr.0)
    }

    #[test]
    pub fn wcstr_to_wide_returns_slice_without_last_nul_test() {
        let data = &[1u16, 1, 1, 1, 1, 0];

        let wcstr = unsafe { WCStr::from_wide_with_nul_unchecked(data) };

        assert_eq!(&[1u16, 1, 1, 1, 1], wcstr.to_wide());
    }

    #[test]
    pub fn wcstr_to_wide_with_nul_returns_slice_with_last_nul_test() {
        let data = &[1u16, 1, 1, 1, 1, 0];

        let wcstr = unsafe { WCStr::from_wide_with_nul_unchecked(data) };

        assert_eq!(&[1u16, 1, 1, 1, 1, 0], wcstr.to_wide_with_nul());
    }
}
