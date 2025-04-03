#![allow(unused)]

use assert_layout::assert_layout;

#[assert_layout(generics = "u8", size = 5, big(generics = "u16", size = 6))]
#[repr(C, packed)]
struct NamespacedStruct<T> {
    #[assert_layout(offset = 0, size = 1, big(offset = 0, size = 2))]
    thing: T,

    #[assert_layout(offset = 1, size = 4, big(offset = 2, size = 4))]
    another: u32,
}
