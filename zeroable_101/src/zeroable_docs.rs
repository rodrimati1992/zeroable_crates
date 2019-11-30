/*!
Documentation for the `Zeroable` macro.

The `Zeroable` derive macro allows deriving the `bytemuck::Zeroable` trait.

# Restrictions

All of these restrictions are enforced at compile-time.

### Structs

None,it works how you'd expect,just do:

```rust
use zeroable_101::Zeroable;

#[derive(Zeroable)]
struct AStruct{
    left:u32,
    right:u32,
}
```

### Enums

Enums must have a `#[repr()]` attribute,
with at least one of `C`/`<some_integer_type>`/`transparent`.

If the enum has a `C` and/or `<some_integer_type>`  representation attribute,
you must either:

- Leave the discriminant of the first variant implicit (it'll be `0`).

- Specify the discriminant of a variant as a literal `0`.

In either case,the fields of the variant with a `0` discriminant
will be required to be zeroable,while the fields of the other variants won't be.

If the enum has a `trasparent` representation,it must only have a single variant,
with a single field.

### Unions

Unions must have either of these:

- A `transparent` representation attribute,and a single field.

- A field with a `#[zero(zeroable)]` attribute.

# Attributes

These are all the attributes for the derive macro,grouped by where they can be used.

## Container attributes

##### `#[zero(bound="Type:ATrait")]`

Adds a bound to the `Zeroable` impl.

##### `#[zero(not_zeroable(TypeParamA,TypeParamB,TypeParamC))]`

Removes the default `Zeroable` bound for one/many type parameters.

##### `#[zero(debug_print)]`

Prints the generated code,stopping compilation.

## Field attributes

##### `#[zero(zeroable)]`

For unions only.

Determines which field of a union is expected to be initialized with zeroes.

# Examples

### Enum

A Result-like enum,with `Ok` as the variant instantiated by `zeroed`.

```rust
use zeroable_101::Zeroable;

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

`#[zero(not_zeroable(T))]``doesn't cause an error because `T` is not in
the variant instantiated with `zeroed`.

```rust
use zeroable_101::Zeroable;

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
use zeroable_101::Zeroable;

#[derive(Debug,PartialEq,Zeroable)]
#[repr(i8)]
enum Ordering{
    Less=-1,
    Equal=0,
    Greater=1,
}

assert_eq!( Ordering::zeroed(), Ordering::Equal );

```

### Struct

A Rectangle type,

```rust
use zeroable_101::Zeroable;

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
use zeroable_101::Zeroable;

use std::num::NonZeroU32;

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

### Union

```rust
use zeroable_101::Zeroable;

#[derive(Zeroable)]
#[repr(C)] // This isn't necessary for Zeroable
union U32OrArray{
    #[zero(zeroable)]
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
use zeroable_101::Zeroable;

#[derive(Zeroable)]
#[zero(not_zeroable(T))]
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
```


*/
