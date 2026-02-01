use tiny_loop::tool::{ToolArgs, tool};

#[tool(name = "custom_tool")]
async fn my_function(#[serde(rename = "customParam")] param: String) -> String {
    format!("Got: {}", param)
}

#[derive(Clone)]
#[allow(dead_code)]
struct MyService;

#[tool]
impl MyService {
    #[name = "custom_method_one"]
    async fn method_one(self, #[serde(rename = "customParam")] param: String) -> String {
        format!("Method one: {}", param)
    }

    #[name = "custom_method_two"]
    async fn method_two(self, param: String) -> String {
        format!("Method two: {}", param)
    }
}

#[test]
fn test_custom_tool_name() {
    assert_eq!(CustomToolArgs::TOOL_NAME, "custom_tool");
}

#[test]
fn test_custom_method_names() {
    assert_eq!(CustomMethodOneArgs::TOOL_NAME, "custom_method_one");
    assert_eq!(CustomMethodTwoArgs::TOOL_NAME, "custom_method_two");
}

#[test]
fn test_serde_rename_function() {
    let json = r#"{"customParam": "test"}"#;
    let args: CustomToolArgs = serde_json::from_str(json).unwrap();
    assert_eq!(args.param, "test");
}

#[test]
fn test_serde_rename_method() {
    let json = r#"{"customParam": "test"}"#;
    let args: CustomMethodOneArgs = serde_json::from_str(json).unwrap();
    assert_eq!(args.param, "test");
}
