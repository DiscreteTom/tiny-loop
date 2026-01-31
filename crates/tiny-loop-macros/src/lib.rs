mod tool;

use crate::tool::tool_impl;
use proc_macro::TokenStream;
use quote::quote;

/// Transforms a function or method into a tool with generated args struct and `ToolArgs` implementation.
///
/// # Usage
///
/// ## Transform a Function
///
/// ```ignore
/// use tiny_loop::{Agent, tool::tool, llm::OpenAIProvider};
///
/// #[tool]
/// async fn get_weather(
///     /// City name
///     city: String,
/// ) -> String {
///     format!("Weather in {}: Sunny", city)
/// }
///
/// let agent = Agent::new(OpenAIProvider::new())
///     .tool(get_weather);
/// ```
///
/// ## Transform Methods
///
/// For methods, use `.bind()`:
///
/// ```ignore
/// #[derive(Clone)]
/// struct Database;
///
/// #[tool]
/// impl Database {
///     /// Query the database
///     async fn query(
///         self,
///         /// SQL query
///         sql: String,
///     ) -> String {
///         format!("Results for: {}", sql)
///     }
/// }
///
/// let db = Database;
/// let agent = Agent::new(OpenAIProvider::new())
///     .bind(db, Database::query);
/// ```
///
/// # Macro Expansion
///
/// ## Transform a Function
///
/// Input function:
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
/// Expands to:
/// ```ignore
/// /// Arguments for the `fetch` tool.
/// #[derive(tiny_loop::serde::Deserialize, tiny_loop::schemars::JsonSchema)]
/// pub struct FetchArgs {
///     /// URL to fetch
///     pub url: String,
/// }
///
/// impl tiny_loop::tool::ToolArgs for FetchArgs {
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
///
/// ## Transform Methods
///
/// Input method:
/// ```ignore
/// impl Database {
///     /// Query database
///     #[tool]
///     pub async fn query(
///         self,
///         /// SQL query
///         sql: String,
///     ) -> String {
///         todo!()
///     }
/// }
/// ```
///
/// Expands to:
/// ```ignore
/// /// Arguments for the `query` tool.
/// #[derive(tiny_loop::serde::Deserialize, tiny_loop::schemars::JsonSchema)]
/// pub struct QueryArgs {
///     /// SQL query
///     pub sql: String,
/// }
///
/// impl tiny_loop::tool::ToolArgs for QueryArgs {
///     const TOOL_NAME: &'static str = "query";
///     const TOOL_DESCRIPTION: &'static str = "Query database";
/// }
///
/// impl Database {
///     /// Query database
///     pub async fn query(self, args: QueryArgs) -> String {
///         let QueryArgs { sql } = args;
///         todo!()
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn tool(_attr: TokenStream, item: TokenStream) -> TokenStream {
    tool_impl(item, quote!(tiny_loop::tool::ToolArgs), quote!(tiny_loop))
}

/// Same as `#[tool]` but uses internal `ToolArgs` path for use within the `tiny-loop` crate.
#[doc(hidden)]
#[proc_macro_attribute]
pub fn tool_internal(_attr: TokenStream, item: TokenStream) -> TokenStream {
    tool_impl(item, quote!(crate::tool::ToolArgs), quote!(crate))
}
