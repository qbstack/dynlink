//! Cross-platform dynamic linking.
//!
//! This library provides a platform-independent API that allows dynamic linking
//! shared objects, and use the data and functions they contains.
//!
//! # Usage
//!
//! With this library shared object functions can be used.
//!
//! ```no_run
//! use std::error;
//!
//! use dynlink::api::Handle;
//!
//! // sum.c
//! //
//! // int sum_of(int a, int b) {
//! //    return a + b;
//! // }
//!
//! fn main() -> Result<(), Box<dyn error::Error>> {
//!     unsafe {
//!         let handle = Handle::open("libsum.so")?;
//!         let sum_fn = handle.lookup::<extern "C" fn(i32, i32) -> i32>("sum_of")?;
//!
//!         println!("{}", sum_fn.apply(|f| f(1, 1)));
//!
//!         Ok(())
//!     }
//! }
//! ```
//!
//! The main purpose of this library is to write safe (or unsafe) wrappers for shared objects.
//!
//! ```no_run
//! use std::error;
//!
//! use dynlink::api::Handle;
//!
//! // sum.c
//! //
//! // int sum_of(int a, int b) {
//! //    return a + b;
//! // }
//!
//! struct MathLib {
//!     // symbols must not outlive the handle that owns them
//!     _handle: Handle,
//!     sum_fn: extern "C" fn(i32, i32) -> i32,
//! }
//!
//! impl MathLib {
//!     pub fn load() -> Result<Self, Box<dyn error::Error>> {
//!         unsafe {
//!             let handle = Handle::open("libsum.so")?;
//!             let sum_fn = handle.lookup("sum_of")?.leak();
//!
//!             Ok(Self {
//!                 _handle: handle,
//!                 sum_fn: sum_fn,
//!             })
//!         }
//!     }
//!
//!     pub fn sum(&self, a: i32, b: i32) -> i32 {
//!         (self.sum_fn)(a, b)
//!     }
//! }
//!
//! fn main() -> Result<(), Box<dyn error::Error>> {
//!     let mathlib = MathLib::load()?;
//!     println!("{}", mathlib.sum(1, 1));
//!
//!     Ok(())
//! }
//! ```
//!
//! Platform-specific APIs are also available in the `platform` module.
//!
//! ```no_run
//! #![cfg(target_os = "linux")]
//!
//! use std::error;
//!
//! use dynlink::platform::{PlatformHandle, RTLD_LAZY, RTLD_LOCAL};
//!
//! fn main() -> Result<(), Box<dyn error::Error>> {
//!     unsafe {
//!         let handle = PlatformHandle::openc(c"libsum.so", RTLD_LOCAL | RTLD_LAZY)?;
//!         let sum_fn = handle.lookupc::<extern "C" fn(i32, i32) -> i32>(c"sum_of")?;
//!
//!         println!("{}", sum_fn.apply(|f| f(1, 1)));
//!
//!         Ok(())
//!     }
//! }
//! ```
//!

pub mod api;
pub mod platform;
