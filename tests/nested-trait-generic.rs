#![allow(unused)]

use assert_layout::assert_layout;

trait TheTrait {
    type Item;
}

impl TheTrait for u16 {
    type Item = u32;
}

#[assert_layout(generics = "u64")]
#[repr(C, packed)]
struct GenericWithNestedTrait<T: NestedTrait> {
    #[assert_layout(offset = 0, size = 8)]
    item: T,
    #[assert_layout(offset = 8, size = 2)]
    nested: T::Nested,
    #[assert_layout(offset = 10, size = 4)]
    deep_nested: <T::Nested as TheTrait>::Item,
}

trait NestedTrait {
    type Nested: TheTrait;
}

impl NestedTrait for u64 {
    type Nested = u16;
}
