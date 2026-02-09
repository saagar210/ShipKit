//! Proc macros for shipkit-core.
use proc_macro::TokenStream;

/// Derive the `Settings` trait for a struct.
///
/// Requires `#[settings(namespace = "...")]` on the struct.
/// Fields can use `#[settings(default = ...)]` to specify defaults.
#[proc_macro_derive(Settings, attributes(settings))]
pub fn derive_settings(input: TokenStream) -> TokenStream {
    let _ = input;
    TokenStream::new()
}
