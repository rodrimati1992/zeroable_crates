#![no_std]
#![cfg_attr(feature = "nightly_testing", feature(arbitrary_enum_discriminant))]
#![cfg_attr(feature = "nightly_testing", feature(transparent_unions))]
#![cfg_attr(feature = "nightly_testing", feature(transparent_enums))]

mod assert_zeroable;

#[cfg(feature = "testing")]
mod tests;

#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }

extern crate self as zeroable_101;

pub use bytemuck::Zeroable;

pub use zeroable_101_derive::Zeroable;

pub use crate::assert_zeroable::AssertZeroable;
