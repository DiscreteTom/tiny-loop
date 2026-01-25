mod tool;

use crate::tool::tool_impl;
use proc_macro::TokenStream;
use quote::quote;

/// Transforms a function into a tool with generated args struct and `FnToolArgs` implementation.
///
/// # Example
///
/// ```ignore
/// /// Fetch a URL.
/// #[tool]
/// pub async fn fetch(
///     /// URL to fetch
///     url: String,
/// ) -> String {
///     todo!()
/// }
/// ```
///
/// Will be transformed to:
/// ```ignore
/// /// Arguments for the `fetch` tool.
/// #[derive(serde::Deserialize, schemars::JsonSchema)]
/// pub struct FetchArgs {
///     /// URL to fetch
///     pub url: String,
/// }
///
/// impl tiny_loop::tool::FnToolArgs for FetchArgs {
///     const TOOL_NAME: &'static str = "fetch";
///     const TOOL_DESCRIPTION: &'static str = "Fetch a URL.";
/// }
///
/// /// Fetch a URL.
/// pub async fn fetch(args: FetchArgs) -> String {
///     let FetchArgs { url } = args;
///     todo!()
/// }
/// ```
#[proc_macro_attribute]
pub fn tool(_attr: TokenStream, item: TokenStream) -> TokenStream {
    tool_impl(item, quote!(tiny_loop::tool::FnToolArgs))
}

/// Same as `#[tool]` but uses internal `FnToolArgs` path for use within the `tiny-loop` crate.
#[doc(hidden)]
#[proc_macro_attribute]
pub fn tool_internal(_attr: TokenStream, item: TokenStream) -> TokenStream {
    tool_impl(item, quote!(crate::tool::FnToolArgs))
}
