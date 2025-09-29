//! WIN32 dynamic linking.
//!
//! This library binds around <libloaderapi.h> and provides a more memory-safe API
//! that allows dynamic linking shared objects, and use the data and functions they contains.

#[cfg(target_os = "windows")]
pub mod symtab;

#[cfg(target_os = "windows")]
pub mod ffi;
