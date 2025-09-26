use std::{ffi, fmt, marker};

use pointersized::PointerSized;

pub struct Win32Symbol<'symtab, T: PointerSized>(
    pub(super) *mut ffi::c_void,
    pub(super) marker::PhantomData<&'symtab T>,
);

impl<'symtab, T: PointerSized> Win32Symbol<'symtab, T> {
    pub(super) unsafe fn from_ptr(ptr: *mut ffi::c_void) -> Self {
        Self(ptr, marker::PhantomData)
    }

    #[inline]
    pub unsafe fn apply<R>(&self, f: impl Fn(T) -> R) -> R {
        f((&self.0 as *const *mut ffi::c_void).cast::<T>().read())
    }

    #[inline]
    pub unsafe fn leak(self) -> T {
        (&self.0 as *const *mut ffi::c_void).cast::<T>().read()
    }

    #[inline]
    pub unsafe fn leak_as_raw(self) -> *mut ffi::c_void {
        self.0
    }
}

unsafe impl<'symtab, T: PointerSized> Send for Win32Symbol<'symtab, T> {}
unsafe impl<'symtab, T: PointerSized> Sync for Win32Symbol<'symtab, T> {}

impl<'symtab, T: PointerSized> Clone for Win32Symbol<'symtab, T> {
    fn clone(&self) -> Self {
        Self(self.0, marker::PhantomData)
    }
}

impl<'symtab, T: PointerSized> fmt::Debug for Win32Symbol<'symtab, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Win32Symbol({:p})", self.0))
    }
}
