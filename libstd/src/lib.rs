#![no_std]
#![feature(use_extern_macros)]
#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(unwind_attributes)]
#![feature(core_panic)]
#![feature(prelude_import)]
#![feature(custom_attribute, attr_literals)]

//#[macro_reexport(assert, assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne, unreachable, unimplemented, write, writeln, try)]
extern crate core as __core;

pub use core::{assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne};

pub use core::iter;
pub use core::clone;
pub use core::marker;
pub use core::ops;
pub use core::fmt;
pub use core::panicking;
//#[lang = "panic_fmt"]
//pub use core::panicking::panic_fmt;
pub use core::result;
pub use core::option;
pub use core::cmp;
pub use core::convert;
pub use core::slice;
pub use core::borrow;
pub use core::mem;
//pub use core::intrinsics;

pub mod prelude;
#[prelude_import]
pub use prelude::v1::*;

pub mod intrinsics {
    extern "C" {
        pub fn abort() -> !;
        /// Returns the square root of an `f32`
        pub fn sqrtf32(x: f32) -> f32;
        pub fn cosf32(x: f32) -> f32;
        pub fn sinf32(x: f32) -> f32;
        pub fn absf32(x: f32) -> f32;
    }
}

pub mod f32 {
    use intrinsics;
    #[lang = "f32_runtime"]
    impl f32 {
        #[inline]
        pub fn sqrt(self) -> f32 {
            unsafe { intrinsics::sqrtf32(self) }
        }

        #[inline]
        pub fn sin(self) -> f32 {
            unsafe { intrinsics::sinf32(self) }
        }

        #[inline]
        pub fn cos(self) -> f32 {
            unsafe { intrinsics::cosf32(self) }
        }

        #[inline]
        pub fn abs(self) -> f32 {
            unsafe { intrinsics::absf32(self) }
        }
    }
}
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}
#[lang = "panic_fmt"]
pub extern "C" fn rust_begin_panic() -> ! {
    unsafe { intrinsics::abort() }
}
#[lang = "start"]
fn lang_start(main: fn(), argc: isize, argv: *const *const u8) -> isize {
    0
}

#[macro_export]
macro_rules! panic {
    () => {
        $crate::rust_begin_panic()
    };
    ($msg:expr) => {
        panic!()
    };
}
