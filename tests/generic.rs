#![allow(unused)]

use assert_layout::assert_layout;

#[assert_layout(size = 9, generics = "u32", generics = "i32")]
#[repr(C, packed)]
struct MyGenericStruct<T> {
    a: u8,
    b: u32,
    #[assert_layout(size = 4)]
    c: T,
}
