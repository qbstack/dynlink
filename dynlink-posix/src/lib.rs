//! POSIX dynamic linking.
//!
//! This library binds around <dlfcn.h> and provides a more memory-safe API
//! that allows dynamic linking shared objects, and use the data and functions they contains.

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
pub mod symtab;
