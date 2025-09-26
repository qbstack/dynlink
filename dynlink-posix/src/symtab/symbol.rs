use std::{ffi, fmt, marker};

use pointersized::PointerSized;

/// Represents a typed symbol from a shared object file's symbol table.
///
/// # Usage
///
/// `PosixSymbol` is used to access and use the functions or data it represents.
///
/// ```no_run
/// use dynlink_posix::symtab::{PosixHandle, PosixSymbol, RTLD_LAZY, RTLD_LOCAL};
///
/// // sum.c
/// //
/// // int sum_of(int a, int b) {
/// //    return a + b;
/// // }
///
/// fn main() {
///     unsafe {
///         let handle = PosixHandle::openc(c"libsum.so", RTLD_LOCAL | RTLD_LAZY)
///             .expect("libsum handle was not opened");
///
///         let symbol: PosixSymbol<'_, extern "C" fn(i32, i32) -> i32> = handle.lookupc(c"sum_of")
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
/// `PosixSymbol` must not outlive the handle that owns it and a type `T` must be ABI
/// compatible with the type of symbol from a shared object.
pub struct PosixSymbol<'symtab, T: PointerSized>(
    pub(super) *mut ffi::c_void,
    pub(super) marker::PhantomData<&'symtab T>,
);

impl<'symtab, T: PointerSized> PosixSymbol<'symtab, T> {
    /// Creates owned symbol from raw pointer.
    pub(super) unsafe fn from_ptr(ptr: *mut ffi::c_void) -> Self {
        Self(ptr, marker::PhantomData)
    }

    /// Applies as the type it represents.
    ///
    /// # Safety
    ///
    /// Type `T` must be ABI compatible with the type of symbol from a shared object.
    #[inline]
    pub unsafe fn apply<R>(&self, f: impl Fn(T) -> R) -> R {
        f((&self.0 as *const *mut ffi::c_void).cast::<T>().read())
    }

    /// Leaks as the type it represents.
    ///
    /// # Safety
    ///
    /// Returning value of the type `T` (which can be copied) must not outlive the handle that owns it.
    /// Type `T` must be ABI compatible with the type of symbol from a shared object.
    #[inline]
    pub unsafe fn leak(self) -> T {
        (&self.0 as *const *mut ffi::c_void).cast::<T>().read()
    }

    /// Leaks as raw pointer.
    ///
    /// # Safety
    ///
    /// Returning pointer must not outlive the handle that owns it.
    #[inline]
    pub unsafe fn leak_as_raw(self) -> *mut ffi::c_void {
        self.0
    }
}

unsafe impl<'symtab, T: PointerSized> Send for PosixSymbol<'symtab, T> {}
unsafe impl<'symtab, T: PointerSized> Sync for PosixSymbol<'symtab, T> {}

impl<'symtab, T: PointerSized> Clone for PosixSymbol<'symtab, T> {
    fn clone(&self) -> Self {
        Self(self.0, marker::PhantomData)
    }
}

impl<'symtab, T: PointerSized> fmt::Debug for PosixSymbol<'symtab, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PosixSymbol({:p})", self.0))
    }
}

#[cfg(test)]
mod unittest {
    use std::{ffi, marker};

    use crate::symtab::PosixSymbol;

    pub fn assert_send<T: Send>() {}
    pub fn assert_sync<T: Sync>() {}

    pub fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    #[test]
    pub fn posix_symbol_marked_as_send_test() {
        assert_send::<PosixSymbol<'_, fn(i32, i32) -> i32>>();
    }

    #[test]
    pub fn posix_symbol_marked_as_sync_test() {
        assert_sync::<PosixSymbol<'_, fn(i32, i32) -> i32>>();
    }

    #[test]
    pub fn posix_symbol_applies_as_type_it_represents_test() {
        unsafe {
            let symbol: PosixSymbol<'_, fn(i32, i32) -> i32> =
                PosixSymbol(sum as *mut ffi::c_void, marker::PhantomData);

            assert_eq!(2, symbol.apply(|it| it(1, 1)));
        }
    }

    #[test]
    pub fn posix_symbol_leaks_as_type_it_reprensents_test() {
        unsafe {
            let symbol: PosixSymbol<'_, fn(i32, i32) -> i32> =
                PosixSymbol(sum as *mut ffi::c_void, marker::PhantomData);

            let sum_fn = symbol.leak();
            assert_eq!(2, sum_fn(1, 1));
        }

        unsafe {
            let symbol: PosixSymbol<'_, *mut ffi::c_void> =
                PosixSymbol(sum as *mut ffi::c_void, marker::PhantomData);

            assert_eq!(sum as *mut ffi::c_void, symbol.leak());
        }
    }

    #[test]
    pub fn posix_symbol_leaks_as_raw_ptr_test() {
        unsafe {
            let symbol: PosixSymbol<'_, fn(i32, i32) -> i32> =
                PosixSymbol(sum as *mut ffi::c_void, marker::PhantomData);

            assert_eq!(sum as *mut ffi::c_void, symbol.leak_as_raw());
        }
    }
}
