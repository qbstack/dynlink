use std::{error, ffi, fmt};

use pointersized::PointerSized;
use windows_sys::Win32::{Foundation, System::LibraryLoader};

use crate::{ffi::WCStr, symtab::Win32Symbol};

pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_IGNORE_CODE_AUTHZ_LEVEL;

pub const LOAD_LIBRARY_AS_DATAFILE: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_AS_DATAFILE;

pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE;

pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_AS_IMAGE_RESOURCE;

pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_SEARCH_APPLICATION_DIR;

pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_SEARCH_DEFAULT_DIRS;

pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR;

pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_SEARCH_SYSTEM32;

pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_SEARCH_USER_DIRS;

pub const LOAD_WITH_ALTERED_SEARCH_PATH: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_WITH_ALTERED_SEARCH_PATH;

pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_REQUIRE_SIGNED_TARGET;

pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LibraryLoader::LOAD_LIBRARY_FLAGS =
    LibraryLoader::LOAD_LIBRARY_SAFE_CURRENT_DIRS;

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

pub enum Win32LinkingError {
    System(Win32SystemCode),
    Unknown,
}

impl Win32LinkingError {
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

pub struct Win32Handle(pub(super) *mut ffi::c_void);

impl Win32Handle {
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
