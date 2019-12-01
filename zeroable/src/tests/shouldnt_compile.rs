//! Tests for things that pass macro expansion but shouldn't compile.
//!

#![allow(dead_code)]

///
/// ```compile_fail
/// #![feature(transparent_unions)]
///
/// use zeroable::Zeroable;
///
/// use core::num::NonZeroU8;
///
/// #[derive(Zeroable)]
/// #[repr(transparent)]
/// union UnionTransparent {
///     #[zero(nonzero)]
///     a: NonZeroU8,
/// }
/// ```
///
/// ```rust
/// #![feature(transparent_unions)]
///
/// use zeroable::Zeroable;
///
/// #[derive(Zeroable)]
/// #[repr(transparent)]
/// union UnionTransparent {
///     a: u8,
/// }
/// ```
#[cfg(feature = "nightly_testing")]
pub struct UnionTransparentNonCompiling;

///
/// ```compile_fail
/// use zeroable::Zeroable;
///
/// #[derive(Zeroable)]
/// union UnionTransparent {
///     a: u8,
///     b: &'static str,
/// }
/// ```
///
/// ```rust
/// use zeroable::Zeroable;
///
/// #[derive(Zeroable)]
/// union UnionTransparent {
///     a: u8,
///     #[zero(nonzero)]
///     b: &'static str,
/// }
/// ```
pub struct UnionNonCompiling;

///////////////////////////////////////////////////////////////////////////////

///
/// ```compile_fail
/// #![feature(transparent_enums)]
///
/// use zeroable::Zeroable;
///
/// use core::num::NonZeroU8;
///
/// #[derive(Zeroable)]
/// #[repr(transparent)]
/// enum Enum {
///     A(NonZeroU8),
/// }
/// ```
///
/// ```rust
/// #![feature(transparent_enums)]
///
/// use zeroable::Zeroable;
///
/// #[derive(Zeroable)]
/// #[repr(transparent)]
/// enum Enum {
///     A(u8),
/// }
/// ```
#[cfg(feature = "nightly_testing")]
pub struct EnumTransparentNonCompiling;

///
/// ```compile_fail
/// use zeroable::Zeroable;
///
/// use core::num::NonZeroU8;
///
/// #[derive(Zeroable)]
/// #[repr(C)]
/// enum Enum {
///     A(NonZeroU8),
///     B(u8),
/// }
/// ```
///
/// ```rust
/// use zeroable::Zeroable;
///
/// use core::num::NonZeroU8;
///
/// #[derive(Zeroable)]
/// #[repr(C)]
/// enum Enum {
///     B(u8),
///     A(NonZeroU8),
/// }
/// ```
pub struct EnumNonCompiling;

///////////////////////////////////////////////////////////////////////////////

///
/// ```compile_fail
/// use zeroable::Zeroable;
///
/// use core::num::NonZeroU16;
///
/// #[derive(Zeroable)]
/// #[repr(transparent)]
/// struct Struct{
///     a:NonZeroU16,
/// }
/// ```
///
/// ```rust
/// use zeroable::Zeroable;
///
/// #[derive(Zeroable)]
/// #[repr(transparent)]
/// struct Struct{
///     a:u16,
/// }
/// ```
pub struct StructNonCompiling;
