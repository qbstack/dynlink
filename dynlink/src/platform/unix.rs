use dynlink_posix::symtab::{PosixHandle, PosixLinkingError, PosixSymbol, PosixSystemMessage};

pub use dynlink_posix::symtab::{RTLD_GLOBAL, RTLD_LAZY, RTLD_LOCAL, RTLD_NOW};

pub type PlatformHandle = PosixHandle;
pub type PlatformSymbol<'symtab, T> = PosixSymbol<'symtab, T>;
pub type PlatformLinkingError = PosixLinkingError;
pub type PlatformMessage = PosixSystemMessage;
