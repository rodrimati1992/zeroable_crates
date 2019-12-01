/*!
Documentation for the `Zeroable` derive macro.

This macro is for deriving the
[`bytemuck::Zeroable` trait](https://docs.rs/bytemuck/1/bytemuck/trait.Zeroable.html).


# Restrictions

All of these restrictions are enforced at compile-time.

### Structs

All fields are required to implement Zeroable.

```rust
use zeroable::Zeroable;

#[derive(Zeroable)]
struct AStruct{
    left:u32,
    right:u32,
}
```

### Enums

Enums must satisfy one of these:

- Having a `#[repr(C/u8/i8/u16/i16/u32/i32/u64/i64/u128/i128/usize/isize)]` attribute,
    with either an implicit discriminant for the first variant (which is always `0`),
    or an explicit `0` discriminant for some variant.
    <br>
    The fields of the variant with a `0` discriminant will then be required to
    implement Zeroable,while the fields of other variants won't be.

- Having a `#[repr(transparent)]` attribute,with a single variant and field,
    which must implement Zeroable.

### Unions

All fields are required to implement Zeroable by default,
opting out of Zeroable for fields individually with `#[zero(nonzero)]`.

The alternative to using the `#[zero(nonzero)]` attribute on fields is
to use the `#[zero(nonzero_fields)]` attribute on the union
(which makes not requiring zeroable for fields the default for that union),
then using the `#[zero(zeroable)]` attribute on zeroable fields.

Zeroable impls for unions have documentation mentioning
which fields were marked as zeroable,and which are not.

# Attributes

These are all the attributes for the derive macro,grouped by where they can be used.

## Container attributes

##### `#[zero(bound="Type:ATrait")]`

Adds a bound to the `Zeroable` impl.

##### `#[zero(not_zeroable(TypeParamA,TypeParamB,TypeParamC))]`

Removes the default `Zeroable` bound for one/many type parameters.

##### `#[zero(nonzero_fields)]`

For unions only.

Marks all the fields as not being zeroable,
requiring some fields to have a `#[zero(zeroable)]` attribute.

##### `#[zero(debug_print)]`

Prints the generated code,stopping compilation.

## Field attributes

##### `#[zero(zeroable)]`

For unions only.

Marks the field as being initializable with zeroes.

The field is then mentioned in the generated documentation for
the Zeroable impl under `Zeroable Fields`.

##### `#[zero(nonzero)]`

For unions only.

Marks the field as not being initializable with zeroes.

The field is then mentioned in the generated documentation for
the Zeroable impl under `NonZero Fields`.

# Examples

### Enum

A Result-like enum,with `Ok` as the variant instantiated by `zeroed`.

```rust
use zeroable::Zeroable;

#[derive(Debug,PartialEq,Zeroable)]
#[repr(u8)]
#[zero(not_zeroable(E))]
enum MyResult<T,E>{
    Ok(T),
    Err(E),
}

assert_eq!( MyResult::<(),String>::zeroed(), MyResult::Ok(()) );
assert_eq!( MyResult::<bool,Vec<()>>::zeroed(), MyResult::Ok(false) );

// This won't compile because Vec is not zeroable.
// assert_eq!( MyResult::<Vec<()>,String>::zeroed(), MyResult::Ok(vec![]) );

```

### Enum

A simple Option-like enum.

In this the None variant is the one instantiated by `zeroed`.

`#[zero(not_zeroable(T))]` doesn't cause an error because `T` is not in
the variant instantiated by `zeroed`
(if `None` contained a `T`,then it would be an error).

```rust
use zeroable::Zeroable;

#[derive(Debug,PartialEq,Zeroable)]
#[repr(u8)]
#[zero(not_zeroable(T))]
enum MyOption<T>{
    None,
    Some(T)
}


assert_eq!( MyOption::<String>::zeroed(), MyOption::None );

```

### Enum

Here is an Ordering-like enum.

```rust
use zeroable::Zeroable;

#[derive(Debug,PartialEq,Zeroable)]
#[repr(i8)]
enum Ordering{
    Less=-1,
    Equal=0,
    Greater=1,
}

assert_eq!( Ordering::zeroed(), Ordering::Equal );

```

### Enum (non-compiling)

This doesn't compile because there is no variant with a `0` discriminant.

```compile_fail
use zeroable::Zeroable;

#[derive(Debug,PartialEq,Zeroable)]
#[repr(u8)]
enum Directions{
    Left=1,
    Right,
    Up,
    Down,
}

```

### Enum (non-compiling)

This doesn't compile because the first variant contains a `NonZeroU8`,
which is not zeroable.

```compile_fail
use zeroable::Zeroable;

use core::num::NonZeroU8;

#[derive(Debug,PartialEq,Zeroable)]
#[repr(u8)]
enum NonZeroOrZeroable{
    NonZero(NonZeroU8),
    Zeroable(u8),
}

```

It compiles if you swap the variants:
```rust
use zeroable::Zeroable;

use core::num::NonZeroU8;

#[derive(Debug,PartialEq,Zeroable)]
#[repr(u8)]
enum NonZeroOrZeroable{
    Zeroable(u8),
    NonZero(NonZeroU8),
}

assert_eq!( NonZeroOrZeroable::zeroed(), NonZeroOrZeroable::Zeroable(0) );

```
this is because the first variant of an enum implicitly has a `0` discriminant,
and `u8` is zeroable.

### Enum (requires nightly)

This is an example of a `#[repr(transparent)]` enum.

*/
#![cfg_attr(feature="nightly_docs",doc="```rust")]
#![cfg_attr(not(feature="nightly_docs"),doc="```ignore")]
/*!
#![feature(transparent_enums)]

use zeroable::Zeroable;

#[derive(Debug,PartialEq,Zeroable)]
#[repr(transparent)]
enum Wrapper<T>{
    Value(T),
}

assert_eq!( Wrapper::<isize>::zeroed(), Wrapper::Value(0_isize) );
assert_eq!( Wrapper::<usize>::zeroed(), Wrapper::Value(0_usize) );
assert_eq!( Wrapper::<(usize,usize)>::zeroed(), Wrapper::Value((0_usize,0_usize)) );
```

### Enum (requires nightly) (non-compiling)

This is an example that fixes a non-compiling enum by setting the discriminant
of a variant to 0.

*/
#![cfg_attr(feature="nightly_docs",doc="```compile_fail")]
#![cfg_attr(not(feature="nightly_docs"),doc="```ignore")]
/*!
use zeroable::Zeroable;

use std::error::Error;

#[derive(Debug,Zeroable)]
#[repr(i8)]
enum MyError{
    PageNotFound{name:String},
    Undefined,
    Other(Box<dyn Error>)
}
```
This fails to compile because String isn't zeroable,
so let's change the variant with a zero discriminant to `Undefined`

*/
#![cfg_attr(feature="nightly_docs",doc="```rust")]
#![cfg_attr(not(feature="nightly_docs"),doc="```ignore")]
/*!
#![feature(arbitrary_enum_discriminant)]

use zeroable::Zeroable;

use std::error::Error;

#[derive(Debug,Zeroable)]
#[repr(i8)]
enum MyError{
    PageNotFound{name:String}=-1,
    Undefined=0,
    Other(Box<dyn Error>),
}
```
The first variant has to have an explicit discriminant,
because otherwise it uses `0` as its discriminant,
causing a compile time error.

### Struct

A Rectangle type.

```rust
use zeroable::Zeroable;

#[derive(Debug,PartialEq,Zeroable)]
struct Rectangle<T>{
    x:T,
    y:T,
    w:T,
    h:T,
}

assert_eq!( Rectangle::zeroed(), Rectangle{ x:0, y:0, w:0, h:0 } );

```

### Struct

Here we define a binary tree of zeroable with indices instead of pointers:

```
use zeroable::Zeroable;

use core::num::NonZeroU32;

#[derive(Debug,PartialEq)]
struct Tree<T>{
    list:Vec<TreeNode<T>>,
}

#[derive(Debug,PartialEq,Zeroable)]
struct TreeNode<T>{
    value:T,
    left:Option<NonZeroU32>,
    right:Option<NonZeroU32>,
}

assert_eq!(
    TreeNode::<[u8;32]>::zeroed(),
    TreeNode{
        value:[0;32],
        left:None,
        right:None,
    },
);

```

### Struct (non-compiling)

This doesn't compile because `&[T]` is not zeroable.

```compile_fail
use zeroable::Zeroable;

#[derive(Debug,PartialEq,Zeroable)]
struct NonEmptySlice<'a,T>{
    slice:&'a [T],
}

```

### Union

```rust
use zeroable::Zeroable;

#[derive(Zeroable)]
#[repr(C)] // This isn't necessary for Zeroable
union U32OrArray{
    num:u32,
    arr:[u8;4],
}

unsafe{
    let zeroed=U32OrArray::zeroed();

    assert_eq!( zeroed.num, 0 );

    assert_eq!( zeroed.arr, [0;4] );
}
```

### Union

```rust
use zeroable::Zeroable;

#[derive(Zeroable)]
#[zero(not_zeroable(T))]
#[zero(nonzero_fields)]
union CondValue<T:Copy>{
    #[zero(zeroable)]
    cond:bool,
    value:T,
}

unsafe{
    let zeroed=CondValue::<&'static str>::zeroed();
    assert_eq!( zeroed.cond, false );
    // You can't read from `zeroed.value` because a reference can't be zeroed.
}
unsafe{
    let zeroed=CondValue::<char>::zeroed();
    assert_eq!( zeroed.cond, false );
    assert_eq!( zeroed.value, '\0' );
}
```

### Union (requires nightly)

This is an example of a `#[repr(transparent)]` union.

*/
#![cfg_attr(feature="nightly_docs",doc="```rust")]
#![cfg_attr(not(feature="nightly_docs"),doc="```ignore")]
/*!
#![feature(transparent_unions)]

use zeroable::Zeroable;

#[derive(Zeroable)]
#[repr(transparent)]
union Wrapper<T:Copy>{
    value:T,
}

unsafe{
    assert_eq!( Wrapper::<isize>::zeroed().value, 0_isize );
    assert_eq!( Wrapper::<usize>::zeroed().value, 0_usize );
    assert_eq!( Wrapper::<(usize,usize)>::zeroed().value, (0_usize,0_usize) );
}
```

### Union (non-compiling)

This doesn't compile because `ManuallyDrop<T>` is not zeroable.

```compile_fail
use zeroable::Zeroable;

use core::mem::ManuallyDrop;

#[derive(Zeroable)]
#[zero(not_zeroable(T))]
union MaybeUninitialized<T:Copy>{
    uninit:(),
    init:ManuallyDrop<T>,
}
```

To fix it simply remove the default `Zeroable` bound on the field like this:
```
use zeroable::Zeroable;

use core::mem::ManuallyDrop;

#[derive(Zeroable)]
#[zero(not_zeroable(T))]
union MaybeUninitialized<T:Copy>{
    uninit:(),
    #[zero(nonzero)]
    init:ManuallyDrop<T>,
}
```



### Union (non-compiling)

This doesn't compile because the union has a `#[zero(nonzero_fields)]` attribute,
but no field has a `#[zero(zeroable)]` attribute.

```compile_fail
use zeroable::Zeroable;

use core::mem::ManuallyDrop;

#[derive(Zeroable)]
#[zero(not_zeroable(T))]
#[zero(nonzero_fields)]
union UnsafeEither<T:Copy,U:Copy>{
    left:T,
    right:U,
}
```

To fix this instance simply add a `#[zero(zeroable)]` attribute to a field like this:
```
use zeroable::Zeroable;

use core::mem::ManuallyDrop;

#[derive(Zeroable)]
#[zero(not_zeroable(T))]
#[zero(nonzero_fields)]
union UnsafeEither<T:Copy,U:Copy>{
    left:T,
    #[zero(zeroable)]
    right:U,
}
```



*/
