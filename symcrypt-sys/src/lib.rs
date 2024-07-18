#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

extern crate libc;

#[cfg(not(feature = "bindgen"))]
mod symcrypt_bindings;
#[cfg(not(feature = "bindgen"))]
pub use symcrypt_bindings::*;

#[cfg(feature = "bindgen")]
pub use symcrypt_bindgen::*;
