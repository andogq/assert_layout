#![allow(unused)]

use assert_layout::assert_layout;

#[assert_layout(size = 9)]
#[repr(C, packed)]
struct MyStruct {
    #[assert_layout(size = 1, offset = 0)]
    a: u8,
    #[assert_layout(size = 4, offset = 1)]
    b: u32,
    c: u32,
}
