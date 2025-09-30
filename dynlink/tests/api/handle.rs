use dynlink::api::Handle;

#[cfg(all(target_os = "linux", target_arch = "x86"))]
pub const LIBSUM: &'static str = "tests/resource/libsum-x86.so";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub const LIBSUM: &'static str = "tests/resource/libsum-x86_64.so";

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub const LIBSUM: &'static str = "tests/resource/libsum-aarch64.so";

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub const LIBSUM: &'static str = "tests/resource/libsum-x86_64.dylib";

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const LIBSUM: &'static str = "tests/resource/libsum-aarch64.dylib";

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub const LIBSUM: &'static str = "tests/resource/libsum-x86.dll";

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub const LIBSUM: &'static str = "tests/resource/libsum-x86_64.dll";

#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
pub const LIBSUM: &'static str = "tests/resource/libsum-aarch64.dll";

#[cfg(target_os = "linux")]
pub const LIBUNKNOWN: &'static str = "tests/resource/unknown.so";

#[cfg(target_os = "macos")]
pub const LIBUNKNOWN: &'static str = "tests/resource/unknown.dylib";

#[cfg(target_os = "windows")]
pub const LIBUNKNOWN: &'static str = "tests/resource/unknown.dll";

pub const SYMBOL_SUM: &'static str = "sum_of";

pub const SYMBOL_UNKNOWN: &'static str = "unknown";

#[test]
pub fn handle_opens_when_path_exists() {
    unsafe {
        let _ = Handle::open(LIBSUM).expect("Shared object was not opened");
    }
}

#[test]
pub fn handle_fails_to_open_when_path_does_not_exist() {
    unsafe {
        let _ = Handle::open(LIBUNKNOWN).expect_err("Unknow shared object was opened");
    }
}

#[test]
pub fn handle_finds_symbol_when_symbol_exists() {
    unsafe {
        let lib = Handle::open(LIBSUM).expect("Shared object was not opened");

        let sum_fn = lib
            .lookup::<extern "C" fn(i32, i32) -> i32>(SYMBOL_SUM)
            .expect("Symbol was not found");

        assert_eq!(2, sum_fn.apply(|it| it(1, 1)));
    }
}

#[test]
pub fn handle_fails_to_find_symbol_when_symbol_does_not_exist() {
    unsafe {
        let lib = Handle::open(LIBSUM).expect("Shared object was not opened");

        let _ = lib
            .lookup::<extern "C" fn(i32, i32) -> i32>(SYMBOL_UNKNOWN)
            .expect_err("Unknow symbol was found");
    }
}
