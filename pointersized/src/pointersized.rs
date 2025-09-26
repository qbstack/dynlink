use std::ffi;

/// Pointer-sized types marker.
///
/// This trait is implemented for opaque c-pointers and functions.
///
/// # Example
///
/// ```no_run
/// use std::ffi;
///
/// use pointersized::PointerSized;
///
/// fn assert_pointer_sized<T: PointerSized>() {}
///
/// fn main() {
///     assert_pointer_sized::<*const ffi::c_void>();
///     assert_pointer_sized::<*mut ffi::c_void>();
///     assert_pointer_sized::<fn(i32, i32) -> i32>();
/// }
/// ```
pub trait PointerSized {}

impl PointerSized for *const ffi::c_void {}
impl PointerSized for *mut ffi::c_void {}

macro_rules! impl_pointer_sized {
    ($($abi:tt)*) => {
        impl<R> PointerSized for $($abi)*() -> R {}

        impl<R, A1> PointerSized for $($abi)*(A1) -> R {}

        impl<R, A1, A2> PointerSized for $($abi)*(A1, A2) -> R {}

        impl<R, A1, A2, A3> PointerSized for $($abi)*(A1, A2, A3) -> R {}

        impl<R, A1, A2, A3, A4> PointerSized for $($abi)*(A1, A2, A3, A4) -> R {}

        impl<R, A1, A2, A3, A4, A5> PointerSized for $($abi)*(A1, A2, A3, A4, A5) -> R {}

        impl<R, A1, A2, A3, A4, A5, A6> PointerSized for $($abi)*(A1, A2, A3, A4, A5, A6) -> R {}

        impl<R, A1, A2, A3, A4, A5, A6, A7> PointerSized for $($abi)*(A1, A2, A3, A4, A5, A6, A7) -> R {}

        impl<R, A1, A2, A3, A4, A5, A6, A7, A8> PointerSized for $($abi)*(
            A1, A2, A3, A4, A5, A6, A7, A8
        ) -> R {}

        impl<R, A1, A2, A3, A4, A5, A6, A7, A8, A9> PointerSized for $($abi)*(
            A1, A2, A3, A4, A5, A6, A7, A8, A9
        ) -> R {}

        impl<R, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10> PointerSized for $($abi)*(
            A1, A2, A3, A4, A5, A6, A7, A8, A9, A10
        ) -> R {}

        impl<R, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11> PointerSized for $($abi)*(
            A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11
        ) -> R {}

        impl<R, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12> PointerSized for $($abi)*(
            A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12
        ) -> R {}

        impl<R, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13> PointerSized for $($abi)*(
            A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13
        ) -> R {}

        impl<R, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14> PointerSized for $($abi)*(
            A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14
        ) -> R {}

        impl<R, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15> PointerSized for $($abi)*(
            A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15
        ) -> R {}

        impl<R, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16> PointerSized for $($abi)*(
            A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16
        ) -> R {}
    };
}

impl_pointer_sized!(fn);
impl_pointer_sized!(extern "C" fn);
impl_pointer_sized!(unsafe extern "C" fn);
impl_pointer_sized!(extern "system" fn);
impl_pointer_sized!(unsafe extern "system" fn);

#[cfg(test)]
mod unittest {
    use std::ffi;

    use crate::PointerSized;

    pub const fn assert_pointer_sized<T: PointerSized>() {}

    #[test]
    pub fn const_raw_pointer_marked_as_pointer_sized() {
        assert_pointer_sized::<*const ffi::c_void>();
    }

    #[test]
    pub fn mut_raw_pointer_marked_as_pointer_sized() {
        assert_pointer_sized::<*mut ffi::c_void>();
    }

    #[test]
    pub fn functions_marked_as_pointer_sized() {
        assert_pointer_sized::<fn()>();
        assert_pointer_sized::<fn() -> i32>();
        assert_pointer_sized::<fn(i32) -> i32>();
        assert_pointer_sized::<fn(i32, i32) -> i32>();
        assert_pointer_sized::<fn(i32, i32, i32) -> i32>();
        assert_pointer_sized::<fn(i32, i32, i32, i32) -> i32>();
        assert_pointer_sized::<fn(i32, i32, i32, i32, i32) -> i32>();
        assert_pointer_sized::<fn(i32, i32, i32, i32, i32, i32) -> i32>();
    }

    #[test]
    pub fn c_abi_functions_marked_as_pointer_sized() {
        assert_pointer_sized::<extern "C" fn()>();
        assert_pointer_sized::<unsafe extern "C" fn()>();

        assert_pointer_sized::<extern "C" fn() -> i32>();
        assert_pointer_sized::<unsafe extern "C" fn() -> i32>();

        assert_pointer_sized::<extern "C" fn(i32) -> i32>();
        assert_pointer_sized::<unsafe extern "C" fn(i32) -> i32>();

        assert_pointer_sized::<extern "C" fn(i32, i32) -> i32>();
        assert_pointer_sized::<unsafe extern "C" fn(i32, i32) -> i32>();

        assert_pointer_sized::<extern "C" fn(i32, i32, i32) -> i32>();
        assert_pointer_sized::<unsafe extern "C" fn(i32, i32, i32) -> i32>();

        assert_pointer_sized::<extern "C" fn(i32, i32, i32, i32) -> i32>();
        assert_pointer_sized::<unsafe extern "C" fn(i32, i32, i32, i32) -> i32>();

        assert_pointer_sized::<extern "C" fn(i32, i32, i32, i32, i32) -> i32>();
        assert_pointer_sized::<unsafe extern "C" fn(i32, i32, i32, i32, i32) -> i32>();

        assert_pointer_sized::<extern "C" fn(i32, i32, i32, i32, i32, i32) -> i32>();
        assert_pointer_sized::<unsafe extern "C" fn(i32, i32, i32, i32, i32, i32) -> i32>();
    }

    #[test]
    pub fn system_abi_functions_marked_as_pointer_sized() {
        assert_pointer_sized::<extern "system" fn()>();
        assert_pointer_sized::<unsafe extern "system" fn()>();

        assert_pointer_sized::<extern "system" fn() -> i32>();
        assert_pointer_sized::<unsafe extern "system" fn() -> i32>();

        assert_pointer_sized::<extern "system" fn(i32) -> i32>();
        assert_pointer_sized::<unsafe extern "system" fn(i32) -> i32>();

        assert_pointer_sized::<extern "system" fn(i32, i32) -> i32>();
        assert_pointer_sized::<unsafe extern "system" fn(i32, i32) -> i32>();

        assert_pointer_sized::<extern "system" fn(i32, i32, i32) -> i32>();
        assert_pointer_sized::<unsafe extern "system" fn(i32, i32, i32) -> i32>();

        assert_pointer_sized::<extern "system" fn(i32, i32, i32, i32) -> i32>();
        assert_pointer_sized::<unsafe extern "system" fn(i32, i32, i32, i32) -> i32>();

        assert_pointer_sized::<extern "system" fn(i32, i32, i32, i32, i32) -> i32>();
        assert_pointer_sized::<unsafe extern "system" fn(i32, i32, i32, i32, i32) -> i32>();

        assert_pointer_sized::<extern "system" fn(i32, i32, i32, i32, i32, i32) -> i32>();
        assert_pointer_sized::<unsafe extern "system" fn(i32, i32, i32, i32, i32, i32) -> i32>();
    }
}
