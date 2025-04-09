mod foreach;
use foreach::ForEachIntTypeInput;

use proc_macro::TokenStream;
use quote::quote;

// for_each_int_type!(#( path_to_macro!( #type ) )*);
// for_each_int_type!(path_to_macro!( #( #type )* ))

/// Takes as input a path to a macro and optional modifiers. Additionally, you can also add the `args:` or `each:`
/// modifiers to the beginning of the input in order to control how the macro is invoked. With `each:`, the macro
/// is invoked for each type. With `args:`, the macro is invoked with each type as input (without separators).
/// The default mode is `each:`.
/// # Example
/// ```rust,no_run
/// for_each_int_type!(each:path_to_macro);
/// // or
/// for_each_int_type!(args: path_to_macro);
/// // or
/// for_each_int_type!(path_to_macro; signed);
/// // or
/// for_each_int_type!(path_to_macro; signed !sized !(64, 128));
/// ```
/// # Modifiers
/// - `none`
/// - `all`
/// - `deterministic` (all types besides `isize` and `usize`)
/// - `sized` (`isize` and `usize`)
/// - `signed`
/// - `unsigned`
/// - `8` (`u8` and `i8`)
/// - `16` (`u16` and `i16`)
/// - `32` (`u32` and `i32`)
/// - `64` (`u64` and `i64`)
/// - `128` (`u128` and `i128`)
/// - `u8`
/// - `u16`
/// - `u32`
/// - `u64`
/// - `u128`
/// - `usize`
/// - `i8`
/// - `i16`
/// - `i32`
/// - `i64`
/// - `i128`
/// - `isize`
/// 
/// Each modifier can be negated using `!`.
/// ```rust, no_run
/// for_each_int_type!(no_sized; all !sized);
/// ```
/// 
/// Modifiers can also be placed in groups, that can also be negated:
/// ```rust, no_run
/// for_each_int_type!(groups; signed !(isize i128) (u8 u16))
/// ```
/// When adding modifiers, the modifiers start out with no types applied.
/// You must add them with modifiers. A good start is with the `all` mask,
/// which sets all the flags. Then you can remove flags that you don't want.
#[proc_macro]
pub fn for_each_int_type(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as ForEachIntTypeInput);
    quote!( #parsed ).into()
}