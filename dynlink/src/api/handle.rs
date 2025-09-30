use std::{error, ffi, fmt};

use crate::{
    api::Symbol,
    platform::{PlatformHandle, PlatformLinkingError, PlatformMessage},
};

pub enum LinkingError {
    System(PlatformMessage),
    Unknown,
}

impl LinkingError {
    pub(super) fn from(err: PlatformLinkingError) -> Self {
        match err {
            PlatformLinkingError::System(msg) => Self::System(msg),
            PlatformLinkingError::Unknown => Self::Unknown,
        }
    }
}

impl Clone for LinkingError {
    fn clone(&self) -> Self {
        match self {
            Self::System(msg) => Self::System(msg.clone()),
            Self::Unknown => Self::Unknown,
        }
    }
}

impl fmt::Debug for LinkingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::System(msg) => f.write_fmt(format_args!("System({:?})", msg)),
            Self::Unknown => f.write_str("Unknown"),
        }
    }
}

impl fmt::Display for LinkingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkingError::System(msg) => {
                f.write_fmt(format_args!("Error occurred dynamic linking: {}", msg))
            }

            LinkingError::Unknown => f.write_str("Error occurred dynamic linking: unknown"),
        }
    }
}

impl error::Error for LinkingError {}

pub struct Handle(pub(super) PlatformHandle);

impl Handle {
    pub unsafe fn open(path: impl AsRef<ffi::OsStr>) -> Result<Self, LinkingError> {
        match PlatformHandle::open(path) {
            Ok(handle) => Ok(Self(handle)),
            Err(err) => Err(LinkingError::from(err)),
        }
    }

    pub unsafe fn lookup<T: pointersized::PointerSized>(
        &self,
        symbol: &str,
    ) -> Result<Symbol<'_, T>, LinkingError> {
        match self.0.lookup(symbol) {
            Ok(symbol) => Ok(Symbol(symbol)),
            Err(err) => Err(LinkingError::from(err)),
        }
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}
