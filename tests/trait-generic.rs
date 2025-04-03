#![allow(unused)]

use assert_layout::assert_layout;

#[assert_layout(generics = "u16")]
#[repr(C, packed)]
struct GenericWithTrait<T: TheTrait> {
    #[assert_layout(offset = 0, size = 2)]
    item: T,
    #[assert_layout(offset = 2, size = 4)]
    nested: T::Item,
}

trait TheTrait {
    type Item;
}

impl TheTrait for u16 {
    type Item = u32;
}
