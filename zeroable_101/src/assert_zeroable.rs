use crate::Zeroable;

use bytemuck::Pod;

use core::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
};

/// A marker type representing that `T` is `Zeroable`.
///
/// This type is zero-sized.
#[repr(C)]
pub struct AssertZeroable<T: ?Sized>(PhantomData<T>);

const _ZERO_SIZED:[();1-core::mem::size_of::<AssertZeroable<()>>()]=[()];


impl<T> AssertZeroable<T>
where
    T: Zeroable + ?Sized,
{
    /// Constructs a `AssertZeroable<T>`
    pub const NEW: Self = AssertZeroable(PhantomData);
}

impl<T> AssertZeroable<T> {
    /// Gets a zeroed `T`.
    ///
    /// This is safe to call,
    /// since constructing a `AssertZeroable<T>` requires that `T` is `Zeroable`.
    #[inline(always)]
    pub fn zeroed(self) -> T {
        unsafe { mem::zeroed() }
    }
}

unsafe impl<T> Zeroable for AssertZeroable<T> where T: Zeroable {}

unsafe impl<T> Pod for AssertZeroable<T> where T: Pod {}

impl<T: ?Sized> Copy for AssertZeroable<T> {}

impl<T: ?Sized> Clone for AssertZeroable<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> PartialEq for AssertZeroable<T> {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T: ?Sized> Eq for AssertZeroable<T> {}

impl<T: ?Sized> PartialOrd for AssertZeroable<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: ?Sized> Ord for AssertZeroable<T> {
    #[inline]
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<T: ?Sized> Hash for AssertZeroable<T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        ().hash(state)
    }
}

impl<T: ?Sized> Debug for AssertZeroable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("AssertZeroable");

        #[cfg(feature = "print_type")]
        let ds = ds.field("type", &std::any::type_name::<T>());

        ds.finish()
    }
}
