/*!
A bytemuck adjacent library,with a derive macro for `Zeroable`.

# Derive Documentation

[Here is the documentation for `Zeroable`](./zeroable_docs/index.html)



*/

#![no_std]
#![cfg_attr(feature = "nightly_testing", feature(arbitrary_enum_discriminant))]
#![cfg_attr(feature = "nightly_testing", feature(transparent_unions))]
#![cfg_attr(feature = "nightly_testing", feature(transparent_enums))]

mod assert_zeroable;

pub mod zeroable_docs;

extern crate self as zeroable_101;

#[doc(noinline)]
pub use bytemuck;

/// A reexport of the `bytemuck::Zeroable` trait.
///
pub use bytemuck::Zeroable;

pub use zeroable_101_derive::Zeroable;

pub use crate::assert_zeroable::AssertZeroable;


#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }

#[cfg(feature = "testing")]
mod tests;




