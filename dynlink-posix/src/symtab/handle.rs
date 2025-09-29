use std::{error, ffi, fmt, os::unix::ffi::OsStrExt};

use pointersized::PointerSized;
use smallvec;

use crate::symtab::PosixSymbol;

/// Lazy symbol resolution option.
///
/// Relocations will be performed at an implementation-defined time, ranging from the time
/// of the `PosixHandle::openc` call until the first reference to a given symbol occurs.
///
/// Specifying `RTLD_LAZY` should improve performance on implementations supporting dynamic
/// symbol binding since a process might not reference all of the symbols in a symbol table.
/// And, for systems supporting dynamic symbol resolution for normal process execution,
/// this behavior mimics the normal handling of process execution.
///
/// # Notes
///
/// Conflicts with `RTLD_NOW`.
pub const RTLD_LAZY: ffi::c_int = libc::RTLD_LAZY;

/// Eager symbol resolution option.
///
/// All necessary relocations will be performed when the symbol table is first loaded.
///
/// This may waste some processing if relocations are performed for symbols that are never referenced.
/// This behavior may be useful for applications that need to know that all symbols referenced
/// during execution will be available before `PosixHandle::openc` returns.
///
/// # Notes
///
/// Conflicts with `RTLD_LAZY`.
pub const RTLD_NOW: ffi::c_int = libc::RTLD_NOW;

/// Global symbol visibility.
///
/// The shared object file's symbols will be made available for relocation processing
/// of any other executable object file.
///
/// # Notes
///
/// Conflicts with `RTLD_LOCAL`.
pub const RTLD_GLOBAL: ffi::c_int = libc::RTLD_GLOBAL;

/// Local symbol visibility.
///
/// The shared object file's symbols will not be made available for relocation processing
/// of any other executable object file.
///
/// # Notes
///
/// Conflicts with: `RTLD_GLOBAL`.
pub const RTLD_LOCAL: ffi::c_int = libc::RTLD_LOCAL;

/// Represents a system message with diagnostic information.
pub struct PosixSystemMessage(pub(super) ffi::CString);

impl PosixSystemMessage {
    /// Creates owned message cloned from c-str.
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

/// Represents an error that occurred during dynamic linking processing.
pub enum PosixLinkingError {
    System(PosixSystemMessage),
    Unknown,
}

impl PosixLinkingError {
    /// Creates owned error cloned from c-str.
    pub(super) fn clone_from_str(msg: &ffi::CStr) -> Self {
        Self::System(PosixSystemMessage::clone_from_str(msg))
    }

    /// Creates owned error cloned from raw c-str pointer.
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

/// Represents an opaque handle of a shared object file's symbol table.
///
/// # Usage
///
/// `PosixHandle` is used to symbol lookup.
///
/// ```no_run
/// use dynlink_posix::symtab::{PosixHandle, PosixSymbol, RTLD_LAZY, RTLD_LOCAL};
///
/// // sum.c
/// //
/// // int sum_of(int a, int b) {
/// //    return a + b;
/// // }
///
/// fn main() {
///     unsafe {
///         let handle = PosixHandle::openc(c"libsum.so", RTLD_LOCAL | RTLD_LAZY)
///             .expect("libsum handle was not opened");
///
///         let symbol = handle.lookupc::<extern "C" fn(i32, i32) -> i32>(c"sum_of")
///             .expect("sum_of symbol was not found");
///
///         let _ = handle.lookupc::<extern "C" fn(i32, i32) -> i32>(c"unknown")
///             .expect_err("unknown symbol was found");
///     }
/// }
/// ```
///
/// # Safety
///
/// Shared object initialization routines that are executed when a
/// `PosixHandle::openc` is called may contain undefined behavior (UB).
///
/// The thread-safety of `PosixHandle` depends on the `dlfcn` implementation.
/// It is thread-safe only if the implementations of `dlopen`, `dlsym`, `dlclose`, and `dlerror` are thread-safe.
pub struct PosixHandle(pub(super) *mut ffi::c_void);

impl PosixHandle {
    pub unsafe fn open(path: impl AsRef<ffi::OsStr>) -> Result<Self, PosixLinkingError> {
        let path_bytes = path.as_ref().as_bytes();
        let options = RTLD_LAZY | RTLD_LOCAL;

        match ffi::CStr::from_bytes_until_nul(path_bytes) {
            Ok(cpath) => Self::openc(cpath, options),
            Err(_) => {
                const PATH_ESTIMATED_MAX_LEN: usize = 4096;

                let mut buf =
                    smallvec::SmallVec::<[u8; PATH_ESTIMATED_MAX_LEN]>::from_slice(path_bytes);
                buf.push(0);

                let cpath = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(&buf) };
                Self::openc(cpath, options)
            }
        }
    }

    /// Opens shared object file specified by null-terminated `path` and loads it into the process address
    /// space according to `options` and returns an owned handle.
    ///
    /// # Safety
    ///
    /// Shared object initialization routines that are executed when this
    /// function is called may be UB.
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

    pub unsafe fn lookup<T: pointersized::PointerSized>(
        &self,
        symbol: &str,
    ) -> Result<PosixSymbol<'_, T>, PosixLinkingError> {
        let symbol_bytes = symbol.as_bytes();

        match ffi::CStr::from_bytes_until_nul(symbol_bytes) {
            Ok(csymbol) => self.lookupc(csymbol),
            Err(_) => {
                const SYMBOL_ESTIMATED_MAX_LEN: usize = 4096;

                let mut buf =
                    smallvec::SmallVec::<[u8; SYMBOL_ESTIMATED_MAX_LEN]>::from_slice(symbol_bytes);
                buf.push(0);

                let csymbol = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(&buf) };
                self.lookupc(csymbol)
            }
        }
    }

    /// Looks up a symbol from the shared object file's symbol table by null-terminated name.
    ///
    /// # Safety
    ///
    /// Type `T` must be ABI compatible with the type of symbol from the shared object.
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
