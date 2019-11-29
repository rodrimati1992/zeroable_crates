use crate::Zeroable;

use core::{
    fmt::{self,Debug},
    marker::PhantomData,
    mem,
};


/// A value level assertion that `T:Zeroable`.
#[derive(Eq,PartialEq,Ord,PartialOrd,Hash)]
pub struct AssertZeroable<T:?Sized>(PhantomData<T>);


impl<T:?Sized> Copy for AssertZeroable<T>{}

impl<T:?Sized> Clone for AssertZeroable<T>{
    fn clone(&self)->Self{
        *self
    }
}

impl<T:?Sized> Debug for AssertZeroable<T>{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        let mut ds=f.debug_struct("AssertZeroable");
        #[cfg(feature="print_type")]
        let ds=ds.field("type",&std::any::type_name::<T>());
        ds.finish()
    }
}


impl<T> AssertZeroable<T>
where
    T:Zeroable+?Sized
{
    /// Constructs a `AssertZeroable<T>`
    pub const NEW:Self=AssertZeroable(PhantomData);
}

impl<T> AssertZeroable<T>{
    /// Gets a zeroed `T`.
    /// 
    /// This is safe to call,
    /// since constructing a `AssertZeroable<T>``requires proving that `T` is `Zeroable`.
    #[inline(always)]
    pub fn zeroed(self)->T{
        unsafe{
            mem::zeroed()
        }
    }
}
