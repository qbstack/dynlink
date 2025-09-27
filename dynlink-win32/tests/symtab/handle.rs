use std::{ffi, os::windows::ffi::OsStrExt};

use dynlink_win32::{ffi::WCStr, symtab::Win32Handle};

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub const LIBSUM: &'static str = "tests/resource/libsum-x86.dll";

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub const LIBSUM: &'static str = "tests/resource/libsum-x86_64.dll";

#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
pub const LIBSUM: &'static str = "tests/resource/libsum-aarch64.dll";

pub const LIBUNKNOWN: &'static str = "tests/resource/unknown.dll";

pub const SYMBOL_SUM: &'static ffi::CStr = c"sum_of";

pub const SYMBOL_UNKNOWN: &'static ffi::CStr = c"unknown";

fn encode_wide_with_nul<'ws>(str: &str, buf: &'ws mut Vec<u16>) -> &'ws WCStr {
    let mut iter = ffi::OsStr::new(str).encode_wide();

    while let Some(codepoint) = iter.next() {
        buf.push(codepoint);
    }
    buf.push(0);

    unsafe { WCStr::from_wide_with_nul_unchecked(buf) }
}

#[test]
pub fn win32_handle_opens_when_path_exists() {
    unsafe {
        let mut buf = vec![];
        let wpath = encode_wide_with_nul(LIBSUM, &mut buf);

        let _ = Win32Handle::openwc(wpath, 0).expect("Shared object was not opened");
    }
}

#[test]
pub fn win32_handle_fails_to_open_when_path_does_not_exist() {
    unsafe {
        let mut buf = vec![];
        let wpath = encode_wide_with_nul(LIBUNKNOWN, &mut buf);

        let _ = Win32Handle::openwc(wpath, 0).expect_err("Unknow shared object was opened");
    }
}

#[test]
pub fn win32_handle_finds_symbol_when_symbol_exists() {
    unsafe {
        let mut buf = vec![];
        let wpath = encode_wide_with_nul(LIBSUM, &mut buf);

        let lib = Win32Handle::openwc(wpath, 0).expect("Shared object was not opened");

        let sum_fn = lib
            .lookupc::<extern "C" fn(i32, i32) -> i32>(SYMBOL_SUM)
            .expect("Symbol was not found");

        assert_eq!(2, sum_fn.apply(|it| it(1, 1)));
    }
}

#[test]
pub fn win32_handle_fails_to_find_symbol_when_symbol_does_not_exist() {
    unsafe {
        let mut buf = vec![];
        let wpath = encode_wide_with_nul(LIBSUM, &mut buf);

        let lib = Win32Handle::openwc(wpath, 0).expect("Shared object was not opened");

        let _ = lib
            .lookupc::<extern "C" fn(i32, i32) -> i32>(SYMBOL_UNKNOWN)
            .expect_err("Unknow symbol was found");
    }
}
