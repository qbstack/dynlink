#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "solaris",
    target_os = "illumos",
    target_os = "haiku",
))]
mod unix;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "solaris",
    target_os = "illumos",
    target_os = "haiku",
    target_os = "windows"
)))]
mod noop;

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "solaris",
    target_os = "illumos",
    target_os = "haiku",
))]
pub use unix::{
    PlatformHandle, PlatformLinkingError, PlatformMessage, PlatformSymbol, RTLD_GLOBAL, RTLD_LAZY,
    RTLD_LOCAL, RTLD_NOW,
};

#[cfg(target_os = "windows")]
pub use windows::{
    PlatformHandle, PlatformLinkingError, PlatformMessage, PlatformSymbol,
    LOAD_IGNORE_CODE_AUTHZ_LEVEL, LOAD_LIBRARY_AS_DATAFILE, LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE,
    LOAD_LIBRARY_AS_IMAGE_RESOURCE, LOAD_LIBRARY_REQUIRE_SIGNED_TARGET,
    LOAD_LIBRARY_SAFE_CURRENT_DIRS, LOAD_LIBRARY_SEARCH_APPLICATION_DIR,
    LOAD_LIBRARY_SEARCH_DEFAULT_DIRS, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR,
    LOAD_LIBRARY_SEARCH_SYSTEM32, LOAD_LIBRARY_SEARCH_USER_DIRS, LOAD_WITH_ALTERED_SEARCH_PATH,
};

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "solaris",
    target_os = "illumos",
    target_os = "haiku",
    target_os = "windows"
)))]
pub use noop::{PlatformHandle, PlatformLinkingError, PlatformMessage, PlatformSymbol};
