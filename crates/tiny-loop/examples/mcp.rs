mod common;

use common::run_cli_loop;
use rmcp::{
    ServiceExt,
    model::CallToolRequestParams,
    transport::{ConfigureCommandExt, TokioChildProcess},
};
use tiny_loop::{
    Agent,
    llm::OpenAIProvider,
    types::{Parameters, ToolDefinition, ToolFunction},
};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a server running as a child process
    let service = ()
        .serve(TokioChildProcess::new(Command::new("npx").configure(
            |cmd| {
                cmd.args(&["-y", "@modelcontextprotocol/server-filesystem", "."]);
            },
        ))?)
        .await?;
    println!("Connected to MCP server");

    // List available tools and convert to tool definitions
    let tools = service.list_tools(Default::default()).await?.tools;
    let names = tools.iter().map(|t| t.name.to_string()).collect::<Vec<_>>();
    println!("Available tools: {names:#?}");
    let mcp_tool_defs = tools
        .iter()
        .map(|t| ToolDefinition {
            tool_type: "function".into(),
            function: ToolFunction {
                name: t.name.to_string(),
                description: t.description.as_deref().unwrap_or_default().to_string(),
                parameters: Parameters::from_object(t.input_schema.as_ref().clone()),
            },
        })
        .collect();

    let mcp_tool_executor = {
        let peer = service.clone();
        move |name: String, args: String| {
            let peer = peer.clone();
            async move {
                peer.call_tool(CallToolRequestParams {
                    meta: None,
                    name: name.into(),
                    arguments: serde_json::from_str(&args).unwrap(),
                    task: None,
                })
                .await
                .unwrap()
                .content[0]
                    .as_text()
                    .unwrap()
                    .text
                    .clone()
            }
        }
    };

    let api_key = std::env::var("LLM_API_KEY").expect("LLM_API_KEY not set");

    let llm = OpenAIProvider::new()
        .api_key(api_key)
        .base_url("https://openrouter.ai/api/v1")
        .model("google/gemini-3-flash-preview");

    let agent = Agent::new(llm)
        .system("You are a helpful assistant")
        .external(mcp_tool_defs, mcp_tool_executor);

    run_cli_loop(agent).await;

    // Gracefully close the connection
    service.cancel().await?;
    Ok(())
}
