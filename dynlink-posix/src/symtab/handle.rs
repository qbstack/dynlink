use std::{error, ffi, fmt};

use pointersized::PointerSized;

use crate::symtab::PosixSymbol;

pub const RTLD_LAZY: ffi::c_int = libc::RTLD_LAZY;
pub const RTLD_NOW: ffi::c_int = libc::RTLD_NOW;
pub const RTLD_GLOBAL: ffi::c_int = libc::RTLD_GLOBAL;
pub const RTLD_LOCAL: ffi::c_int = libc::RTLD_LOCAL;

pub struct PosixSystemMessage(pub(super) ffi::CString);

impl PosixSystemMessage {
    pub(super) fn clone_from_str(msg: &ffi::CStr) -> Self {
        Self(msg.to_owned())
    }
}

impl Clone for PosixSystemMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for PosixSystemMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", &self.0))
    }
}

impl fmt::Display for PosixSystemMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.0.to_string_lossy()))
    }
}

pub enum PosixLinkingError {
    System(PosixSystemMessage),
    Unknown,
}

impl PosixLinkingError {
    pub(super) fn clone_from_str(msg: &ffi::CStr) -> Self {
        Self::System(PosixSystemMessage::clone_from_str(msg))
    }

    pub(super) unsafe fn clone_from_ptr(msg: *const ffi::c_char) -> Self {
        if !msg.is_null() {
            Self::clone_from_str(ffi::CStr::from_ptr(msg))
        } else {
            Self::Unknown
        }
    }
}

impl Clone for PosixLinkingError {
    fn clone(&self) -> Self {
        match self {
            Self::System(msg) => Self::System(msg.clone()),
            Self::Unknown => Self::Unknown,
        }
    }
}

impl fmt::Debug for PosixLinkingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::System(msg) => f.write_fmt(format_args!("System({:?})", msg)),
            Self::Unknown => f.write_str("Unknown"),
        }
    }
}

impl fmt::Display for PosixLinkingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PosixLinkingError::System(msg) => {
                f.write_fmt(format_args!("Error occurred dynamic linking: {}", msg))
            }

            PosixLinkingError::Unknown => f.write_str("Error occurred dynamic linking: unknown"),
        }
    }
}

impl error::Error for PosixLinkingError {}

pub struct PosixHandle(pub(super) *mut ffi::c_void);

impl PosixHandle {
    pub unsafe fn openc(path: &ffi::CStr, options: ffi::c_int) -> Result<Self, PosixLinkingError> {
        #[cfg(target_os = "freebsd")]
        let _ = libc::dlerror();

        let handle = libc::dlopen(path.as_ptr(), options);

        if !handle.is_null() {
            Ok(Self(handle))
        } else {
            let err = libc::dlerror();
            Err(PosixLinkingError::clone_from_ptr(err))
        }
    }

    pub unsafe fn lookupc<T: PointerSized>(
        &self,
        symbol: &ffi::CStr,
    ) -> Result<PosixSymbol<'_, T>, PosixLinkingError> {
        #[cfg(target_os = "freebsd")]
        let _ = libc::dlerror();

        let ptr = libc::dlsym(self.0, symbol.as_ptr());

        #[cfg(any(
            target_os = "linux",
            target_os = "android",
            target_os = "macos",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "dragonfly",
            target_os = "solaris",
            target_os = "illumos",
            target_os = "haiku",
        ))]
        if !ptr.is_null() {
            Ok(PosixSymbol::from_ptr(ptr))
        } else {
            let err = libc::dlerror();
            if err.is_null() {
                Ok(PosixSymbol::from_ptr(ptr))
            } else {
                Err(PosixLinkingError::clone_from_ptr(err))
            }
        }

        #[cfg(target_os = "freebsd")]
        if !ptr.is_null() {
            Ok(PosixSymbol::from_ptr(ptr))
        } else {
            let err = libc::dlerror();
            Err(PosixLinkingError::clone_from_ptr(err))
        }
    }
}

unsafe impl Send for PosixHandle {}
unsafe impl Sync for PosixHandle {}

impl Drop for PosixHandle {
    fn drop(&mut self) {
        unsafe { libc::dlclose(self.0) };
    }
}

impl fmt::Debug for PosixHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PosixHandle({:p})", self.0))
    }
}

#[cfg(test)]
mod unittest {
    use crate::symtab::PosixHandle;

    pub fn assert_send<T: Send>() {}
    pub fn assert_sync<T: Sync>() {}

    #[test]
    pub fn posix_handle_marked_as_send_test() {
        assert_send::<PosixHandle>();
    }

    #[test]
    pub fn posix_handle_marked_as_sync_test() {
        assert_sync::<PosixHandle>();
    }
}
