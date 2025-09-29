use std::{error, ffi, fmt};

use pointersized::PointerSized;
use windows_sys::Win32::{Foundation, System::LibraryLoader};

use crate::{ffi::WCStr, symtab::Win32Symbol};

/// Ignore restriction policy option.
///
/// If this value is used, the system does not check AppLocker rules or apply
/// Software Restriction Policies for the DLL. This action applies only to the DLL
/// being loaded and not to its dependencies. This value is recommended for use in
/// setup programs that must run extracted DLLs during installation.
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_IGNORE_CODE_AUTHZ_LEVEL;

/// Load as data file option.
///
/// If this value is used, the system maps the file into the calling process's virtual
/// address space as if it were a data file. Nothing is done to execute or prepare to
/// execute the mapped file. Therefore, you cannot call functions like `Win32Handle::lookup`.
pub const LOAD_LIBRARY_AS_DATAFILE: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_AS_DATAFILE;

/// Load as data file exclusive option.
///
/// Similar to `LOAD_LIBRARY_AS_DATAFILE`, except that the DLL file is opened with
/// exclusive write access for the calling process. Other processes cannot open the
/// DLL file for write access while it is in use. However, the DLL can still be opened
/// by other processes.
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE;

/// Load as image option.
///
/// If this value is used, the system maps the file into the process's virtual address
/// space as an image file. However, the loader does not load the static imports or
/// perform the other usual initialization steps. Use this flag when you want to load
/// a DLL only to extract messages or resources from it.
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_AS_IMAGE_RESOURCE;

/// Load from application's installation directory.
///
/// If this value is used, the application's installation directory is searched for the
/// DLL and its dependencies. Directories in the standard search path are not
/// searched.
///
/// # Notes
///
/// Conflicts with `LOAD_WITH_ALTERED_SEARCH_PATH`.
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_SEARCH_APPLICATION_DIR;

/// Load from default directories.
///
/// This value is a combination of `LOAD_LIBRARY_SEARCH_APPLICATION_DIR`,
/// `LOAD_LIBRARY_SEARCH_SYSTEM32`, and `LOAD_LIBRARY_SEARCH_USER_DIRS`.
/// Directories in the standard search path are not searched.
///
/// # Notes
///
/// Conflicts with `LOAD_WITH_ALTERED_SEARCH_PATH`.
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_SEARCH_DEFAULT_DIRS;

/// If this value is used, the directory that contains the DLL is temporarily added to
/// the beginning of the list of directories that are searched for the DLL's
/// dependencies. Directories in the standard search path are not searched.
///
/// # Notes
///
/// Conflicts with `LOAD_WITH_ALTERED_SEARCH_PATH`.
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR;

/// Load from system directories.
///
/// If this value is used, `%windows%\system32` is searched for the DLL and its
/// dependencies. Directories in the standard search path are not searched.
///
/// # Notes
///
/// Conflicts with `LOAD_WITH_ALTERED_SEARCH_PATH`.
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_SEARCH_SYSTEM32;

/// Load from user specified directories.
///
/// If this value is used, directories added using the `Win32::AddDllDirectory` or the
/// `Win32::SetDllDirectory` function are searched for the DLL and its dependencies. If more
/// than one directory has been added, the order in which the directories are
/// searched is unspecified. Directories in the standard search path are not searched.
///
/// # Notes
///
/// Conflicts with `LOAD_WITH_ALTERED_SEARCH_PATH`.
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_SEARCH_USER_DIRS;

/// If this value is used and `path`` specifies an absolute path, the system uses
/// the alternate file search strategy discussed in the Remarks section to find
/// associated executable modules that the specified module causes to be loaded.
/// If this value is used and `path` specifies a relative path, the behavior is
/// undefined.
///
/// If this value is not used, or if `path` does not specify a path, the system
/// uses the standard search strategy discussed in the Remarks section to find
/// associated executable modules that the specified module causes to be loaded.
///
/// # Notes
///
/// Conflicts with any other `LOAD_LIBRARY_SEARCH_*` options.
pub const LOAD_WITH_ALTERED_SEARCH_PATH: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_WITH_ALTERED_SEARCH_PATH;

/// Specifies that the digital signature of the binary image must be checked at load time.
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_REQUIRE_SIGNED_TARGET;

/// If this value is used, loading a DLL for execution from the current directory
/// is only allowed if it is under a directory in the Safe load list.
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_SAFE_CURRENT_DIRS;

/// Represents a system error code.
pub struct Win32SystemCode(pub(super) Foundation::WIN32_ERROR);

impl Clone for Win32SystemCode {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for Win32SystemCode {}

impl fmt::Debug for Win32SystemCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("WIN32_ERROR {:?}", self.0))
    }
}

impl fmt::Display for Win32SystemCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("WIN32_ERROR {}", self.0))
    }
}

/// Represents an error that occurred during dynamic linking processing.
pub enum Win32LinkingError {
    System(Win32SystemCode),
    Unknown,
}

impl Win32LinkingError {
    /// Creates owned error from system error code.
    pub(super) fn from_raw_code(code: Foundation::WIN32_ERROR) -> Self {
        match code {
            0 => Win32LinkingError::Unknown,
            _ => Win32LinkingError::System(Win32SystemCode(code)),
        }
    }
}

impl Clone for Win32LinkingError {
    fn clone(&self) -> Self {
        match self {
            Self::System(code) => Self::System(*code),
            Self::Unknown => Self::Unknown,
        }
    }
}

impl fmt::Debug for Win32LinkingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::System(code) => f.write_fmt(format_args!("System({:?})", code)),
            Self::Unknown => f.write_str("Unknown"),
        }
    }
}

impl fmt::Display for Win32LinkingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Win32LinkingError::System(code) => {
                f.write_fmt(format_args!("Error occurred dynamic linking: {}", code))
            }

            Win32LinkingError::Unknown => f.write_str("Error occurred dynamic linking: unknown"),
        }
    }
}

impl error::Error for Win32LinkingError {}

/// Represents an opaque handle of a shared object file's symbol table.
///
/// # Usage
///
/// `Win32Handle` is used to symbol lookup.
///
/// ```no_run
/// use std::{ffi, os::windows::ffi::OsStrExt};
///
/// use dynlink_win32::{ffi::WCStr, symtab::Win32Handle};
///
/// // sum.c
/// //
/// // int sum_of(int a, int b) {
/// //    return a + b;
/// // }
///
/// fn main() {
///     unsafe {
///         let encoded = ffi::OsStr::new("libsum.dll")
///             .encode_wide()
///             .chain(Some(0))
///             .collect::<Vec<u16>>();
///
///         let wpath = WCStr::from_wide_with_nul(&encoded)
///             .expect("Unreachable");
///
///         let handle = Win32Handle::openwc(wpath, 0)
///             .expect("libsum handle was not opened");
///
///         let _ = handle.lookupc::<extern "C" fn(i32, i32) -> i32>(c"sum_of")
///             .expect("sum_of symbol was not found");
///     }
/// }
/// ```
///
/// # Safety
///
/// Shared object initialization routines that are executed when a
/// `Win32Handle::openwc` is called may contain undefined behavior (UB).
///
/// The thread-safety of `Win32Handle` depends on the `libloaderapi` implementation.
pub struct Win32Handle(pub(super) *mut ffi::c_void);

impl Win32Handle {
    /// Opens shared object file specified by null-terminated `path` and loads it into the process address
    /// space according to `options` and returns an owned handle.
    ///
    /// # Safety
    ///
    /// Shared object initialization routines that are executed when this
    /// function is called may be UB.
    pub unsafe fn openwc(
        path: &WCStr,
        options: LibraryLoader::LOAD_LIBRARY_FLAGS,
    ) -> Result<Self, Win32LinkingError> {
        let handle = LibraryLoader::LoadLibraryExW(path.as_ptr(), 0 as *mut ffi::c_void, options);

        if !handle.is_null() {
            Ok(Self(handle))
        } else {
            let err = Foundation::GetLastError();
            Err(Win32LinkingError::from_raw_code(err))
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
    ) -> Result<Win32Symbol<'_, T>, Win32LinkingError> {
        let ptr = LibraryLoader::GetProcAddress(self.0, symbol.as_ptr() as *const u8);

        match ptr {
            Some(addr) => Ok(Win32Symbol::from_ptr(addr as *mut ffi::c_void)),
            None => {
                let err = Foundation::GetLastError();
                Err(Win32LinkingError::from_raw_code(err))
            }
        }
    }
}

unsafe impl Send for Win32Handle {}
unsafe impl Sync for Win32Handle {}

impl Drop for Win32Handle {
    fn drop(&mut self) {
        unsafe { Foundation::FreeLibrary(self.0) };
    }
}

impl fmt::Debug for Win32Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Win32Handle({:p})", self.0))
    }
}

#[cfg(test)]
mod unittest {
    use crate::symtab::Win32Handle;

    pub fn assert_send<T: Send>() {}
    pub fn assert_sync<T: Sync>() {}

    #[test]
    pub fn win32_handle_marked_as_send_test() {
        assert_send::<Win32Handle>();
    }

    #[test]
    pub fn win32_handle_marked_as_sync_test() {
        assert_sync::<Win32Handle>();
    }
}
