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
