#![allow(unused)]

use assert_layout::assert_layout;

#[assert_layout(
    generics = "u8",
    generics = "u16",
    little(generics = "u8", size = 5),
    big(generics = "u16", size = 6)
)]
#[repr(C, packed)]
struct NamespacedStruct2<T> {
    #[assert_layout(offset = 0, little(size = 1), big(size = 2))]
    thing: T,

    #[assert_layout(size = 4, little(offset = 1), big(offset = 2))]
    another: u32,
}
