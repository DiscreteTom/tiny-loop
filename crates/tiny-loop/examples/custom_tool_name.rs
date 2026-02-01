use tiny_loop::tool::{ToolArgs, tool};

#[tool(name = "custom_tool")]
async fn my_function(#[serde(rename = "customParam")] param: String) -> String {
    format!("Got: {}", param)
}

#[derive(Clone)]
struct MyService;

#[tool]
impl MyService {
    #[name = "custom_method_one"]
    async fn method_one(self, param: String) -> String {
        format!("Method one: {}", param)
    }

    #[name = "custom_method_two"]
    async fn method_two(self, param: String) -> String {
        format!("Method two: {}", param)
    }
}

#[tokio::main]
async fn main() {
    // Verify the generated struct name and tool name
    println!("Tool name: {}", CustomToolArgs::TOOL_NAME);
    assert_eq!(CustomToolArgs::TOOL_NAME, "custom_tool");

    println!("Method one tool name: {}", CustomMethodOneArgs::TOOL_NAME);
    println!("Method two tool name: {}", CustomMethodTwoArgs::TOOL_NAME);
    assert_eq!(CustomMethodOneArgs::TOOL_NAME, "custom_method_one");
    assert_eq!(CustomMethodTwoArgs::TOOL_NAME, "custom_method_two");

    println!("Test passed!");
}
