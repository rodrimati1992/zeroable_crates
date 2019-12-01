/*!
A [bytemuck](https://docs.rs/bytemuck/1) adjacent library,with a derive macro for
[`Zeroable`](https://docs.rs/bytemuck/1/bytemuck/trait.Zeroable.html).

# Derive Documentation

[Here is the documentation for the `Zeroable` derive macro
](./zeroable_docs/index.html)

# Examples

### Structs


[Here are more struct examples
](https://docs.rs/zeroable/0.2/zeroable/zeroable_docs/index.html#struct)

```rust

use zeroable::Zeroable;

#[derive(Debug,PartialEq,Zeroable)]
struct Point3D<T>{
    x:T,
    y:T,
    z:T,
}

assert_eq!( Point3D::zeroed() , Point3D{ x:0, y:0, z:0 } );

```

### Enums

There are some restrictions for enums,documented in
[the Zeroable macro docs
](https://docs.rs/zeroable/0.2/zeroable/zeroable_docs/index.html#enums).

[Here are more enum examples
](https://docs.rs/zeroable/0.2/zeroable/zeroable_docs/index.html#enum)


```rust

use zeroable::Zeroable;

#[derive(Debug,PartialEq,Zeroable)]
#[repr(u8)]
enum Ternary{
    Undefined,
    False,
    True,
}

assert_eq!( Ternary::zeroed() , Ternary::Undefined );

```

### Unions

There are some restrictions for unions,documented in
[the Zeroable macro docs
](https://docs.rs/zeroable/0.2/zeroable/zeroable_docs/index.html#unions).

[Here are more union examples
](https://docs.rs/zeroable/0.2/zeroable/zeroable_docs/index.html#union)


```rust

use zeroable::Zeroable;

#[derive(Zeroable)]
union Signedness{
    signed:i8,
    unsigned:u8,
}

unsafe{
    assert_eq!( Signedness::zeroed().signed , 0_i8 );
    assert_eq!( Signedness::zeroed().unsigned , 0_u8 );
}

```

# Features

- "print_type":
    Slightly improved debugging,
    shows the type of `T` in `AssertZeroable<T>`'s ' Debug implementation

- "nightly_docs":
    Makes the documentation examples that require Rust nightly run in doctests,
    and shows them as tested in the documentation.

# `#[no_std]` support

This crate is `#[no_std]`,and only requires the `core` library.


*/

#![cfg_attr(not(feature = "nightly_docs"), no_std)]
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
