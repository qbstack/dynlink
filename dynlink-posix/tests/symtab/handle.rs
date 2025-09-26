use std::ffi;

use dynlink_posix::symtab::{PosixHandle, RTLD_LAZY, RTLD_LOCAL};

#[cfg(all(target_os = "linux", target_arch = "x86"))]
pub const LIBSUM: &'static ffi::CStr = c"tests/resource/libsum-x86.so";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub const LIBSUM: &'static ffi::CStr = c"tests/resource/libsum-x86_64.so";

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub const LIBSUM: &'static ffi::CStr = c"tests/resource/libsum-aarch64.so";

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub const LIBSUM: &'static ffi::CStr = c"tests/resource/libsum-x86_64.dylib";

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const LIBSUM: &'static ffi::CStr = c"tests/resource/libsum-aarch64.dylib";

#[cfg(target_os = "linux")]
pub const LIBUNKNOWN: &'static ffi::CStr = c"tests/resource/unknown.so";

#[cfg(target_os = "macos")]
pub const LIBUNKNOWN: &'static ffi::CStr = c"tests/resource/unknown.dylib";

pub const SYMBOL_SUM: &'static ffi::CStr = c"sum_of";

pub const SYMBOL_UNKNOWN: &'static ffi::CStr = c"unknown";

#[test]
pub fn posix_handle_opens_when_path_exists() {
    unsafe {
        let _ = PosixHandle::openc(LIBSUM, RTLD_LOCAL | RTLD_LAZY)
            .expect("Shared object was not opened");
    }
}

#[test]
pub fn posix_handle_fails_to_open_when_path_does_not_exist() {
    unsafe {
        let _ = PosixHandle::openc(LIBUNKNOWN, RTLD_LOCAL | RTLD_LAZY)
            .expect_err("Unknow shared object was opened");
    }
}

#[test]
pub fn posix_handle_finds_symbol_when_symbol_exists() {
    unsafe {
        let lib = PosixHandle::openc(LIBSUM, RTLD_LOCAL | RTLD_LAZY)
            .expect("Shared object was not opened");

        let sum_fn = lib
            .lookupc::<extern "C" fn(i32, i32) -> i32>(SYMBOL_SUM)
            .expect("Symbol was not found");

        assert_eq!(2, sum_fn.apply(|it| it(1, 1)));
    }
}

#[test]
pub fn posix_handle_fails_to_find_symbol_when_symbol_does_not_exist() {
    unsafe {
        let lib = PosixHandle::openc(LIBSUM, RTLD_LOCAL | RTLD_LAZY)
            .expect("Shared object was not opened");

        let _ = lib
            .lookupc::<extern "C" fn(i32, i32) -> i32>(SYMBOL_UNKNOWN)
            .expect_err("Unknow symbol was found");
    }
}
