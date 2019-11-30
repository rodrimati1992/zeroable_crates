[![](https://img.shields.io/crates/v/zeroable.svg)][crates-io]
[![](https://docs.rs/zeroable/badge.svg)][api-docs]

[crates-io]: https://crates.io/crates/zeroable
[api-docs]: https://docs.rs/zeroable



Provides a derive macro for 
[`bytemuck::Zeroable`](https://docs.rs/bytemuck/1/bytemuck/trait.Zeroable.html).

# Derive Documentation

[Here is the documentation for `Zeroable`](https://docs.rs/zeroable/*/zeroable_docs/index.html)

# Examples

### Structs


[Here are more struct examples
](https://docs.rs/zeroable/*/zeroable_docs/index.html#struct)

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
[the Zeroable macro docs](https://docs.rs/zeroable/*/zeroable_docs/index.html#enums).

[Here are more enum examples](https://docs.rs/zeroable/*/zeroable_docs/index.html#enum)


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
[the Zeroable macro docs](https://docs.rs/zeroable/*/zeroable_docs/index.html#unions).

[Here are more union examples](https://docs.rs/zeroable/*/zeroable_docs/index.html#union)


```rust

use zeroable::Zeroable;

#[derive(Zeroable)]
union Signedness{
    #[zero(zeroable)]
    signed:i8,

    unsigned:u8,
}

unsafe{
    assert_eq!( Signedness::zeroed().signed , 0_i8 );
    assert_eq!( Signedness::zeroed().unsigned , 0_u8 );
}

```


# `#[no_std]` support

This crate is `#[no_std]`,and only requires the `core` library.

# Changelog

The changelog is in the "Changelog.md" file.

# Minimum Rust version

This crate support Rust back to 1.34.


