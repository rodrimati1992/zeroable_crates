use crate::Zeroable;

#[derive(Zeroable)]
#[repr(transparent)]
union UnionTransparent {
    a: usize,
}

#[derive(Debug, PartialEq, Zeroable)]
#[repr(transparent)]
enum EnumTransparent {
    A(usize),
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn zeroing_transparent_enums_unions() {
    unsafe {
        assert_eq!(0, UnionTransparent::zeroed().a);
    }

    assert_eq!(EnumTransparent::A(0), EnumTransparent::zeroed());
}
