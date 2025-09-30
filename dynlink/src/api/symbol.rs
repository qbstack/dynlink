use std::{ffi, fmt};

use pointersized::PointerSized;

use crate::platform::PlatformSymbol;

pub struct Symbol<'symtab, T: PointerSized>(pub(super) PlatformSymbol<'symtab, T>);

impl<'symtab, T: PointerSized> Symbol<'symtab, T> {
    #[inline]
    pub unsafe fn apply<R>(&self, f: impl Fn(T) -> R) -> R {
        self.0.apply(f)
    }

    #[inline]
    pub unsafe fn leak(self) -> T {
        self.0.leak()
    }

    #[inline]
    pub unsafe fn leak_as_raw(self) -> *mut ffi::c_void {
        self.0.leak_as_raw()
    }
}

impl<'symtab, T: PointerSized> Clone for Symbol<'symtab, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'symtab, T: PointerSized> fmt::Debug for Symbol<'symtab, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?})", self.0))
    }
}

#[cfg(test)]
mod unittest {
    use crate::api::Symbol;

    pub fn assert_send<T: Send>() {}
    pub fn assert_sync<T: Sync>() {}

    #[test]
    pub fn symbol_marked_as_send_test() {
        assert_send::<Symbol<'_, fn(i32, i32) -> i32>>();
    }

    #[test]
    pub fn symbol_marked_as_sync_test() {
        assert_sync::<Symbol<'_, fn(i32, i32) -> i32>>();
    }
}
