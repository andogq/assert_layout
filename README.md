# assert_layout

***Assert struct layouts, including field sizes and offsets.***

`assert_layout` provides a single proc-macro which will perform compile-time assertions for certain
qualities of a struct. In addition to field size and offset, it's also possible to provide concrete
types for generics and assert that the layout still holds.

## Overview

Struct containers can use the `size` assertion to assert the size of the struct.

```rust
use assert_layout::assert_layout;

#[assert_layout(size = 1)]
#[repr(packed)]
struct MyStruct {
    field: u8,
}
```

Similarly, each field can use the `size` and `offset` assertions.

```rust
use assert_layout::assert_layout;

#[assert_layout(size = 5)]
#[repr(packed)]
struct MyStruct {
    #[assert_layout(offset = 0, size = 1)]
    field: u8,

    #[assert_layout(offset = 2, size = 4)]
    another_field: f32,
}
```

### Generics

If the struct contains generics, concrete types can be provided for assertions using `generics`.
For instance, the following assertion would expand with `T = u32`.

```rust
use assert_layout::assert_layout;

#[assert_layout(size = 4, generics = "u32")]
#[repr(packed)]
struct MyStruct<T> {
    #[assert_layout(offset = 0, size = 4)]
    field: T,
}
```

Multiple generic parameters can by provided with commas between them, in addition to consts (in the
same manner as `MyStruct<...>`).

```rust
use assert_layout::assert_layout;

#[assert_layout(size = 64, generics = "8, f64")]
#[repr(packed)]
struct MyStruct<const N: usize, T> {
    #[assert_layout(offset = 0, size = 64)]
    field: [T; N],
}
```

The `generics` attribute can be provided multiple times in order to assert different combinations
of generics.

```rust
use assert_layout::assert_layout;

#[assert_layout(size = 12, generics = "u32, u64", generics = "i32, i64")]
#[repr(packed)]
struct MyStruct<T, U> {
    #[assert_layout(offset = 0, size = 4)]
    field: T,

    #[assert_layout(offset = 4, size = 8)]
    another_field: U,
}
```

### Namespaces

Sometimes generic combinations will not be compatible with each other, so namespaces can be used to
isolate the assertions. A namespace is created using `namespace(...)`, where `namespace` is a
unique identifier for the namepsace, and the previously described assertions (`generics`, `offset`,
`size`) are within the brackets. Everything within the namespace will be asserted in isolation.

```rust
use assert_layout::assert_layout;

#[assert_layout(little(size = 4, generics = "u32"), big(size = 8, generics = "u64"))]
#[repr(packed)]
struct MyStruct<T> {
    #[assert_layout(little(offset = 0, size = 4), big(offset = 0, size = 8))]
    field: T,
}
```

If there are common assertions between the namespaces (such as `offset = 0` above), they can be
omitted from the namespace and asserted at the top level.

```rust
use assert_layout::assert_layout;

#[assert_layout(
    generics = "u32",
    generics = "u64",
    little(size = 4, generics = "u32"),
    big(size = 8, generics = "u64")
)]
#[repr(packed)]
struct MyStruct<T> {
    #[assert_layout(offset = 0, little(size = 4), big(size = 8))]
    field: T,
}
```
