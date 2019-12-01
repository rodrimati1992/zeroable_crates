/*!
A [bytemuck](https://docs.rs/bytemuck/1) adjacent library,with a derive macro for `Zeroable`.

# Derive Documentation

[Here is the documentation for `Zeroable`](./zeroable_docs/index.html)

# `#[no_std]` support

This crate is `#[no_std]`,and only requires the `core` library.



*/

#![no_std]
#![cfg_attr(feature = "nightly_testing", feature(arbitrary_enum_discriminant))]
#![cfg_attr(feature = "nightly_testing", feature(transparent_unions))]
#![cfg_attr(feature = "nightly_testing", feature(transparent_enums))]

pub mod assert_zeroable;

pub mod zeroable_docs;

extern crate self as zeroable;

#[doc(noinline)]
pub use bytemuck;

/// A reexport of the
/// [`bytemuck::Zeroable`](https://docs.rs/bytemuck/1/bytemuck/trait.Zeroable.html)
/// trait.
///
pub use bytemuck::Zeroable;

pub use zeroable_derive::Zeroable;

pub use crate::assert_zeroable::{AssertZeroable, GetAssertZeroable};

#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }

#[cfg(feature = "testing")]
mod tests;


