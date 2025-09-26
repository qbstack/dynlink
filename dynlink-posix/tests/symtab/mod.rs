#[cfg(any(
    all(
        target_os = "linux",
        any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86")
    ),
    all(
        target_os = "macos",
        any(target_arch = "aarch64", target_arch = "x86_64"),
    )
))]
mod handle;
