use schemars::schema_for;
use tiny_loop::tool::{ToolArgs, tool};

/// First line of documentation
/// Second line of documentation
/// Third line of documentation
#[tool]
async fn multi_line_doc(param: String) -> String {
    format!("Got: {}", param)
}

#[tool]
async fn param_multi_line_doc(
    /// First line of param doc
    /// Second line of param doc
    /// Third line of param doc
    param: String,
) -> String {
    format!("Got: {}", param)
}

#[derive(Clone)]
#[allow(dead_code)]
struct Service;

#[tool]
impl Service {
    /// First line of method doc
    /// Second line of method doc
    /// Third line of method doc
    async fn method_with_doc(
        self,
        /// First line of method param doc
        /// Second line of method param doc
        /// Third line of method param doc
        param: String,
    ) -> String {
        format!("Got: {}", param)
    }
}

#[test]
fn test_multi_line_doc_comment() {
    let desc = MultiLineDocArgs::TOOL_DESCRIPTION;
    assert!(
        desc.contains("First line of documentation"),
        "Description should contain first line"
    );
    assert!(
        desc.contains("Second line of documentation"),
        "Description should contain second line"
    );
    assert!(
        desc.contains("Third line of documentation"),
        "Description should contain third line"
    );
}

#[test]
fn test_multi_line_doc_comment_method() {
    let desc = MethodWithDocArgs::TOOL_DESCRIPTION;
    assert!(
        desc.contains("First line of method doc"),
        "Description should contain first line"
    );
    assert!(
        desc.contains("Second line of method doc"),
        "Description should contain second line"
    );
    assert!(
        desc.contains("Third line of method doc"),
        "Description should contain third line"
    );
}

#[test]
fn test_param_multi_line_doc() {
    let schema = schema_for!(ParamMultiLineDocArgs);
    let json = serde_json::to_string(&schema).unwrap();
    assert!(
        json.contains("First line of param doc"),
        "Schema should contain first line of param doc"
    );
    assert!(
        json.contains("Second line of param doc"),
        "Schema should contain second line of param doc"
    );
    assert!(
        json.contains("Third line of param doc"),
        "Schema should contain third line of param doc"
    );
}

#[test]
fn test_method_param_multi_line_doc() {
    let schema = schema_for!(MethodWithDocArgs);
    let json = serde_json::to_string(&schema).unwrap();
    assert!(
        json.contains("First line of method param doc"),
        "Schema should contain first line of method param doc"
    );
    assert!(
        json.contains("Second line of method param doc"),
        "Schema should contain second line of method param doc"
    );
    assert!(
        json.contains("Third line of method param doc"),
        "Schema should contain third line of method param doc"
    );
}
