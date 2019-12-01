use crate::Zeroable;

use core::{
    cmp::PartialEq,
    fmt::Debug,
    num::{NonZeroU64, NonZeroU8},
};

#[cfg(feature = "nightly_testing")]
mod nightly;

////////////////////////////////////////////////////////////////////////////////

#[derive(Zeroable)]
union UnionZeroableField {
    a: usize,
    // Only the `#[zero(zeroable)]` field has to be zeroable
    #[allow(dead_code)]
    #[zero(nonzero)]
    b: NonZeroU8,
}

#[derive(Zeroable)]
#[zero(not_zeroable(U))]
#[zero(nonzero_fields)]
union UnionZeroableFieldB<U: Copy> {
    #[zero(zeroable)]
    a: usize,
    // Only the `#[zero(zeroable)]` field has to be zeroable
    #[allow(dead_code)]
    b: U,
}

#[derive(Zeroable)]
#[zero(not_zeroable(U))]
union UnionZeroableFieldC<T: Copy, U: Copy> {
    a: T,
    // Only the `#[zero(zeroable)]` field has to be zeroable
    #[allow(dead_code)]
    #[zero(nonzero)]
    b: U,
}

fn generic_union_asserts<T, U>()
where
    T: Zeroable + Debug + PartialEq + Copy,
    U: Debug + PartialEq + Copy,
{
    unsafe {
        assert_eq!(0, UnionZeroableFieldB::<U>::zeroed().a);

        assert_eq!(T::zeroed(), UnionZeroableFieldC::<T, U>::zeroed().a);
    }
}

#[test]
fn stable_union_test() {
    unsafe {
        assert_eq!(UnionZeroableField::zeroed().a, 0);
    }

    generic_union_asserts::<u8, NonZeroU8>();
}

#[test]
fn between_union_fields() {
    type TransmuteInt=UnionZeroableFieldC<u32,[u8;4]>;

    unsafe {
        assert_eq!(TransmuteInt::zeroed().a, 0);
        assert_eq!(TransmuteInt::zeroed().b, [0, 0, 0, 0]);
    }
}


////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Zeroable, PartialEq)]
#[repr(u8)]
enum EnumU8 {
    A(usize),

    // Only the variant with a 0 discriminant has to be zeroable
    #[allow(dead_code)]
    B(NonZeroU8),
}

#[repr(C)]
#[derive(Debug, Zeroable, PartialEq)]
#[zero(not_zeroable(U))]
enum EnumC<T, U> {
    A(T),
    // Only the variant with a 0 discriminant has to be zeroable
    #[allow(dead_code)]
    B(U),
}

#[derive(Debug, Zeroable, PartialEq)]
#[repr(C, usize)]
enum EnumCUsize {
    A(usize),
}

#[derive(Debug, Zeroable, PartialEq)]
#[repr(i8)]
enum EnumPrimitiveI8 {
    #[allow(dead_code)]
    A = 1,
    B = 0,
}

fn generic_enum_asserts<T, U>()
where
    T: Zeroable + Debug + PartialEq,
    U: Debug + PartialEq,
{
    assert_eq!(EnumC::A(T::zeroed()), <EnumC<T, U> as Zeroable>::zeroed());
}

#[test]
fn stable_enum_test() {
    assert_eq!(EnumCUsize::zeroed(), EnumCUsize::A(0));
    assert_eq!(EnumU8::zeroed(), EnumU8::A(0));

    assert_eq!(EnumPrimitiveI8::zeroed(), EnumPrimitiveI8::B);

    generic_enum_asserts::<u8, NonZeroU8>();
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Zeroable, PartialEq)]
struct StructNoFields {}

#[derive(Debug, Zeroable, PartialEq)]
struct Struct2Fields<T> {
    a: usize,
    b: T,
}

#[derive(Debug, Zeroable, PartialEq)]
#[zero(bound = "U: Debug")]
#[zero(_test_code = r#"
    impl<T:Debug+PartialEq,U> Struct3Fields<T,U>{
        const HELLO:&'static str="WHAT THE ....";
    }
    ||{
        let _=<T as Zeroable>::zeroed();
        let _=<T as Debug>::fmt;
        let _=<U as Debug>::fmt;
    };
"#)]
#[zero(not_zeroable(U))]
struct Struct3Fields<T: PartialEq, U>
where
    T: Debug,
{
    a: usize,
    b: T,
    c: *const U,
}

fn generic_struct_asserts<T, U>()
where
    T: Zeroable + Debug + PartialEq,
    U: Debug + PartialEq,
{
    assert_eq!(
        Struct2Fields {
            a: 0,
            b: T::zeroed(),
        },
        <Struct2Fields<T> as Zeroable>::zeroed()
    );

    assert_eq!(
        Struct3Fields {
            a: 0,
            b: T::zeroed(),
            c: 0 as *const U,
        },
        <Struct3Fields<T, U> as Zeroable>::zeroed()
    );

    assert_eq!(Struct3Fields::<T, U>::HELLO, "WHAT THE ....");
}

#[test]
fn stable_struct_test() {
    assert_eq!(StructNoFields::zeroed(), StructNoFields {});

    generic_struct_asserts::<u64, NonZeroU64>();
}

////////////////////////////////////////////////////////////////////////////////
