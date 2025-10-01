use std::{error, ffi, fmt, marker};

use pointersized::PointerSized;

pub struct PlatformMessage(marker::PhantomData<()>);

impl Clone for PlatformMessage {
    fn clone(&self) -> Self {
        Self(marker::PhantomData)
    }
}

impl fmt::Debug for PlatformMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("NoopPlatformMessage")
    }
}

impl fmt::Display for PlatformMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Unsupported platform")
    }
}

pub enum PlatformLinkingError {
    System(PlatformMessage),
    Unknown,
}

impl Clone for PlatformLinkingError {
    fn clone(&self) -> Self {
        match self {
            Self::System(msg) => Self::System(msg.clone()),
            Self::Unknown => Self::Unknown,
        }
    }
}

impl fmt::Debug for PlatformLinkingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::System(msg) => f.write_fmt(format_args!("System({:?})", msg)),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl fmt::Display for PlatformLinkingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlatformLinkingError::System(msg) => {
                f.write_fmt(format_args!("Error occurred dynamic linking: {}", msg))
            }

            PlatformLinkingError::Unknown => f.write_str("Error occurred dynamic linking: unknown"),
        }
    }
}

impl error::Error for PlatformLinkingError {}

pub struct PlatformSymbol<'symtab, T: PointerSized>(marker::PhantomData<&'symtab T>);

impl<'symtab, T: PointerSized> PlatformSymbol<'symtab, T> {
    pub unsafe fn apply<R>(&self, _: impl Fn(T) -> R) -> R {
        compile_error!("Unsupported platform")
    }

    pub unsafe fn leak(self) -> T {
        compile_error!("Unsupported platform")
    }

    pub unsafe fn leak_as_raw(self) -> *mut ffi::c_void {
        compile_error!("Unsupported platform")
    }
}

unsafe impl<'symtab, T: PointerSized> Send for PlatformSymbol<'symtab, T> {}
unsafe impl<'symtab, T: PointerSized> Sync for PlatformSymbol<'symtab, T> {}

impl<'symtab, T: PointerSized> Clone for PlatformSymbol<'symtab, T> {
    fn clone(&self) -> Self {
        Self(marker::PhantomData)
    }
}

impl<'symtab, T: PointerSized> fmt::Debug for PlatformSymbol<'symtab, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("NoopPlatformSymbol")
    }
}

pub struct PlatformHandle(marker::PhantomData<()>);

impl PlatformHandle {
    pub unsafe fn open(_: impl AsRef<ffi::OsStr>) -> Result<Self, PlatformLinkingError> {
        compile_error!("Unsupported platform")
    }

    pub unsafe fn lookup<T: pointersized::PointerSized>(
        &self,
        _: &str,
    ) -> Result<PlatformSymbol<'_, T>, PlatformLinkingError> {
        compile_error!("Unsupported platform")
    }
}

unsafe impl Send for PlatformHandle {}
unsafe impl Sync for PlatformHandle {}

impl fmt::Debug for PlatformHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PlatformHandle")
    }
}
