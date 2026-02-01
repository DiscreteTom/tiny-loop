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
/// ## Custom Tool Name
///
/// ```ignore
/// #[tool(name = "weather_api")]
/// async fn get_weather(
///     /// City name
///     city: String,
/// ) -> String {
///     todo!()
/// }
///
/// #[tool]
/// impl Database {
///     #[name = "db_query"]
///     async fn query(self, sql: String) -> String {
///         todo!()
///     }
/// }
/// ```
///
/// ## Serde Attributes
///
/// Serde attributes like `#[serde(rename = "...")]` can be applied to parameters:
///
/// ```ignore
/// #[tool]
/// async fn fetch(
///     #[serde(rename = "URL")]
///     url: String,
/// ) -> String {
///     todo!()
/// }
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
/// #[derive(serde::Deserialize, schemars::JsonSchema)]
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
/// #[derive(serde::Deserialize, schemars::JsonSchema)]
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
pub fn tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    tool_impl(attr, item, quote!(tiny_loop::tool::ToolArgs))
}

/// Same as `#[tool]` but uses internal `ToolArgs` path for use within the `tiny-loop` crate.
#[doc(hidden)]
#[proc_macro_attribute]
pub fn tool_internal(attr: TokenStream, item: TokenStream) -> TokenStream {
    tool_impl(attr, item, quote!(crate::tool::ToolArgs))
}
