mod foreach;
use foreach::ForEachIntTypeInput;

use proc_macro::TokenStream;
use quote::quote;

/// Takes as input a path to a macro and optional flags. Additionally, you can also add the `args:` or `each:`
/// modifiers to the beginning of the input in order to control how the macro is invoked. With `each:`, the macro
/// is invoked for each type. With `args:`, the macro is invoked with each type as input (without separators).
/// The default mode is `each:`.
/// # Example
/// ```rust,no_run
/// for_each_int_type!(each: path_to_macro);
/// // or
/// for_each_int_type!(args: path_to_macro);
/// // or
/// for_each_int_type!(path_to_macro; signed);
/// // or
/// for_each_int_type!(path_to_macro; signed !sized !(64, 128));
/// ```
/// # Flags
/// - `none`
/// - `all`
/// - `deterministic` (all types besides `isize` and `usize`)
/// - `sized` (`isize` and `usize`)
/// - `signed` (`i8`, `i16`, `i32`, `i64`, `i128`, and `isize`)
/// - `unsigned` (`u8`, `u16`, `u32`, `u64`, `u128`, and `usize`)
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
/// Each flag can be negated using `!`.
/// ```rust, no_run
/// for_each_int_type!(no_sized; all !sized);
/// ```
/// 
/// Flags can also be placed in groups, that can also be negated:
/// ```rust, no_run
/// for_each_int_type!(groups; signed !(isize i128) (u8 u16))
/// ```
/// When adding flags, you must add flags first for them to be active.
/// All flags start off inactive. A good initial flag is the `all` flag
/// so that you can negate the flags you don't want.
#[proc_macro]
pub fn for_each_int_type(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as ForEachIntTypeInput);
    quote!( #parsed ).into()
}