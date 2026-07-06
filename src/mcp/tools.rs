use serde_json::Value;


pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub input_schema: Option<Value>,
}