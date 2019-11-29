#![no_std]


mod assert_zeroable;


pub use bytemuck::Zeroable;

pub use crate::{
    assert_zeroable::AssertZeroable,
};