use schemars::{JsonSchema, generate::SchemaSettings};
use serde::Serialize;
use serde_json::{Map, Value};

/// Tool definition for LLM
#[derive(Serialize, Clone, Debug)]
pub struct ToolDefinition {
    /// Type of the tool (typically "function")
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function definition
    pub function: ToolFunction,
}

/// Tool function definition
#[derive(Serialize, Clone, Debug)]
pub struct ToolFunction {
    /// Function name
    pub name: String,
    /// Function description
    pub description: String,
    /// JSON schema for function parameters
    pub parameters: Parameters,
}

/// JSON schema parameters with metadata stripped
#[derive(Serialize, Clone, Debug)]
pub struct Parameters(Map<String, Value>);

impl Parameters {
    /// Create Parameters from a Json object (map)
    pub fn from_object(mut obj: Map<String, Value>) -> Self {
        // Remove `$schema`, `title`, and `description` fields from JSON schema
        obj.remove("$schema");
        obj.remove("title");
        obj.remove("description");

        Self(obj)
    }

    /// Create Parameters from a JsonSchema
    pub fn from_schema(schema: schemars::Schema) -> Self {
        let obj = schema.to_value().as_object().unwrap().clone();
        Self::from_object(obj)
    }

    /// Create Parameters from a type implementing JsonSchema
    pub fn from_type<T: JsonSchema>() -> Self {
        let settings = SchemaSettings::default().with(|s| {
            s.inline_subschemas = true;
        });
        let generator = settings.into_generator();
        let schema = generator.into_root_schema_for::<T>();
        Self::from_schema(schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_definition_serialization() {
        let td = ToolDefinition {
            tool_type: "function".into(),
            function: ToolFunction {
                name: "test".into(),
                description: "desc".into(),
                parameters: Parameters::from_type::<String>(),
            },
        };
        let json = serde_json::to_string(&td).unwrap();
        assert!(json.contains(r#""type":"function"#));
        assert!(json.contains(r#""name":"test"#));
    }
}
