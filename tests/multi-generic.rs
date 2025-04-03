#![allow(unused)]

use assert_layout::assert_layout;

#[assert_layout(size = 6, generics = "u32, u8")]
#[repr(C, packed)]
struct MyDoubleGenericStruct<T, U> {
    #[assert_layout(offset = 0, size = 1)]
    a: u8,
    #[assert_layout(offset = 1, size = 4)]
    b: T,
    #[assert_layout(offset = 5, size = 1)]
    c: U,
}
