This proc macro crate adds the `for_each_int_type` macro, which invokes the input macro for each integer type. This is useful when you want to implement some trait for each integer type.

the syntax is as follows (not real code):
```
for_each_int_type!((mode:)? macro_path (; flags*)?);
```

```rust
pub trait ExampleTrait {
    fn plus_one(self) -> Self;
}

macro_rules! example_macro {
    ( $type:ty ) => {
        impl ExampleTrait for $type {
            fn plus_one(self) -> Self {
                self + 1
            }
        }
    };
}

for_each_int_type!(example_macro);
```

The syntax for the input of `for_each_int_type` has up to three parts.

The default way to pass input is to just pass the path to a macro. This will invoke that macro for each integer type, including `isize` and `usize`.

Optionally, you can include an invocation mode before the macro path. The invocation mode controls how the given macro is invoked. The two modes are `each`, and `args`. These two modes come before the input macro path. The default mode when the mode is not included is `each`. The mode must be followed by a colon `:`.

```rust
macro_rules! each_macro {
    ( $type:ty ) => {
        impl ExampleTrait for $type {
            fn plus_one(self) -> Self {
                self + 1
            }
        }
    };
}

// with `each` mode, the target macro is invoked once for each type.
for_each_int_type!(each:each_macro);

macro_rules! args_macro {
    ( $( $type )* ) => {
        $(
            impl ExampleTrait for $type {
                fn plus_one(self) -> Self {
                    self + 1
                }
            }
        )*
    };
}

// with `args` mode, the target macro is invoked once with each int type as input. No separators.
for_each_int_type!(args:args_macro);
```

Additionally, you can also add optional flags after `;` after the macro path to control which integer types are passed to the input macro.

```rust
// These flags means:
// include signed types
// exclude sized types (isize and usize) as well as 128-bit types (u128, i128)
// include u128
// exclude i8
for_each_int_type!(example_macro; signed !(sized 128) u128 !i8);
```

The flags are evaluated in the order they are written. You can remove a flag at the begging, then add it back at the end. Or add it at the beginning, and remove it at the end.

You can add flags to a group enclosed with `()` to add or negate all of them at once.

```rust
(u128 i128)
```

You can also negate flags to exclude them from the input.

```rust
!i8 !(sized u128, i128)
```

When you are using flags, by default none of the flags are active. You need to add them.

# flags
- `none` (no flags. Not very useful, but included for completeness)
- `all` (all flags)
- `deterministic` (all types besides `isize` and `usize`)
- `sized` (`isize` and `usize`)
- `signed`
- `unsigned`
- `8` (`u8` and `i8`)
- `16` (`u16` and `i16`)
- `32` (`u32` and `i32`)
- `64` (`u64` and `i64`)
- `128` (`u128` and `i128`)
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `usize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `isize`