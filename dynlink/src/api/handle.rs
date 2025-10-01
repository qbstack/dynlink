use std::{error, ffi, fmt};

use crate::{
    api::Symbol,
    platform::{PlatformHandle, PlatformLinkingError, PlatformMessage},
};

/// Represents an error that occurred during dynamic linking processing.
///
/// `LinkingError::System(msg)` contains a diagnostic message provided by the platform.
/// `LinkingError::Unknown` indicates a failure for which no platform message is available.
pub enum LinkingError {
    System(PlatformMessage),
    Unknown,
}

impl LinkingError {
    /// Creates owned error cloned from `PlatformLinkingError`.
    pub(super) fn from(err: PlatformLinkingError) -> Self {
        match err {
            PlatformLinkingError::System(msg) => Self::System(msg),
            PlatformLinkingError::Unknown => Self::Unknown,
        }
    }
}

impl Clone for LinkingError {
    fn clone(&self) -> Self {
        match self {
            Self::System(msg) => Self::System(msg.clone()),
            Self::Unknown => Self::Unknown,
        }
    }
}

impl fmt::Debug for LinkingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::System(msg) => f.write_fmt(format_args!("System({:?})", msg)),
            Self::Unknown => f.write_str("Unknown"),
        }
    }
}

impl fmt::Display for LinkingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkingError::System(msg) => {
                f.write_fmt(format_args!("Error occurred dynamic linking: {}", msg))
            }

            LinkingError::Unknown => f.write_str("Error occurred dynamic linking: unknown"),
        }
    }
}

impl error::Error for LinkingError {}

/// Represents an opaque handle of a shared object file's symbol table.
///
/// # Usage
///
/// `Handle` is used to symbol lookup.
///
/// ```no_run
/// use dynlink::api::Handle;
///
/// // sum.c
/// //
/// // int sum_of(int a, int b) {
/// //    return a + b;
/// // }
///
/// fn main() {
///     unsafe {
///         let handle = Handle::open("libsum.so")
///             .expect("libsum handle was not opened");
///
///         let symbol = handle.lookup::<extern "C" fn(i32, i32) -> i32>("sum_of")
///             .expect("sum_of symbol was not found");
///
///         let _ = handle.lookup::<extern "C" fn(i32, i32) -> i32>("unknown")
///             .expect_err("unknown symbol was found");
///     }
/// }
/// ```
///
/// # Safety
///
/// Shared object initialization routines that are executed when a
/// `Handle::open` is called may contain undefined behavior (UB).
///
/// The thread-safety of `Handle` depends on the platform implementation.
pub struct Handle(pub(super) PlatformHandle);

impl Handle {
    /// Opens shared object file specified by `path` with default options and loads
    /// it into the process address space and returns an owned handle.
    ///
    /// # Safety
    ///
    /// Shared object initialization routines that are executed when this
    /// function is called may be UB.
    pub unsafe fn open(path: impl AsRef<ffi::OsStr>) -> Result<Self, LinkingError> {
        match PlatformHandle::open(path) {
            Ok(handle) => Ok(Self(handle)),
            Err(err) => Err(LinkingError::from(err)),
        }
    }

    /// Looks up a symbol from the shared object file's symbol table by name.
    ///
    /// # Safety
    ///
    /// Type `T` must be ABI compatible with the type of symbol from the shared object.
    pub unsafe fn lookup<T: pointersized::PointerSized>(
        &self,
        symbol: &str,
    ) -> Result<Symbol<'_, T>, LinkingError> {
        match self.0.lookup(symbol) {
            Ok(symbol) => Ok(Symbol(symbol)),
            Err(err) => Err(LinkingError::from(err)),
        }
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

#[cfg(test)]
mod unittest {
    use crate::api::Handle;

    pub fn assert_send<T: Send>() {}
    pub fn assert_sync<T: Sync>() {}

    #[test]
    pub fn handle_marked_as_send_test() {
        assert_send::<Handle>();
    }

    #[test]
    pub fn handle_marked_as_sync_test() {
        assert_sync::<Handle>();
    }
}
