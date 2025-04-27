use proc_macro::TokenStream;

mod stack_trace_debug;

/// Attribute macro to derive [std::fmt::Debug] for the annotated `Error` type.
///
/// The generated `Debug` implementation will print the error in a stack trace style. E.g.:
/// ```plaintext
/// 0: Foo error, at src/common/catalog/src/error.rs:80:10
/// 1: Bar error, at src/common/function/src/error.rs:90:10
/// 2: Root cause, invalid table name, at src/common/catalog/src/error.rs:100:10
/// ```
///
/// Notes on using this macro:
/// - `#[snafu(display)]` must present on each enum variants,
///   and should not include `location` and `source`.
/// - Only our internal error can be named `source`.
///   All external error should be `error` with an `#[snafu(source)]` annotation.
/// - `common_error` crate must be accessible.
#[proc_macro_attribute]
pub fn stack_trace_debug(args: TokenStream, input: TokenStream) -> TokenStream {
    stack_trace_debug::stack_trace_style_impl(args.into(), input.into()).into()
}
