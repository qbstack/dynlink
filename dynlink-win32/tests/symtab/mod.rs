#[cfg(all(
    target_os = "windows",
    any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86")
))]
mod handle;
