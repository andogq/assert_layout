#![allow(unused)]

use assert_layout::assert_layout;

#[assert_layout(size = 123, generics = "123")]
#[repr(C, packed)]
struct ConstStruct<const N: usize>(#[assert_layout(size = 123)] [u8; N]);
