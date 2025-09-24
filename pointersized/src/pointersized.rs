use std::ffi;

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
