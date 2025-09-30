use std::{ffi, fmt};

use pointersized::PointerSized;

use crate::platform::PlatformSymbol;

/// Represents a typed symbol from a shared object file's symbol table.
///
/// # Usage
///
/// `Symbol` is used to access and use the functions or data it represents.
///
/// ```no_run
/// use dynlink::api::{Handle, Symbol};
///
/// // sum.c
/// //
/// // int sum_of(int a, int b) {
/// //    return a + b;
/// // }
///
/// fn main() {
///     unsafe {
///         let handle = Handle::open("libsum.so")
///             .expect("libsum handle was not opened");
///
///         let symbol: Symbol<'_, extern "C" fn(i32, i32) -> i32> = handle.lookup("sum_of")
///             .expect("sum_of symbol was not found");
///
///         let sum = symbol.apply(|sum_of_fn| sum_of_fn(1, 1));
///         assert_eq!(2, sum);
///     }
/// }
/// ```
///
/// # Safety
///
/// `Symbol` must not outlive the handle that owns it and a type `T` must be ABI
/// compatible with the type of symbol from a shared object.
pub struct Symbol<'symtab, T: PointerSized>(pub(super) PlatformSymbol<'symtab, T>);

impl<'symtab, T: PointerSized> Symbol<'symtab, T> {
    /// Applies as the type it represents.
    ///
    /// # Safety
    ///
    /// Type `T` must be ABI compatible with the type of symbol from a shared object.
    #[inline]
    pub unsafe fn apply<R>(&self, f: impl Fn(T) -> R) -> R {
        self.0.apply(f)
    }

    /// Leaks as the type it represents.
    ///
    /// # Safety
    ///
    /// Returning value of the type `T` (which can be copied) must not outlive the handle that owns it.
    /// Type `T` must be ABI compatible with the type of symbol from a shared object.
    #[inline]
    pub unsafe fn leak(self) -> T {
        self.0.leak()
    }

    /// Leaks as raw pointer.
    ///
    /// # Safety
    ///
    /// Returning pointer must not outlive the handle that owns it.
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
