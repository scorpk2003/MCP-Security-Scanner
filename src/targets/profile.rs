use crate::{InputType, MCPTool, ScanRequest, TargetType};


pub struct Profile {
    pub name: String,
    pub root_path: Option<String>,
    pub command: Option<String>,
    pub runtime: McpRuntime,
    pub config: Vec<String>,
    pub env: Vec<String>,
    pub source_files: Vec<String>,
    pub tools: Vec<MCPTool>,
}

impl Profile {
    // pub fn new(input_mode: &InputType) -> Self {
    //     match input_mode {
    //         InputType::TARGET => {},
    //         InputType::COMMAND => {},
    //         InputType::CONFIG => {}
    //     }
    // }
}

pub enum McpRuntime {
    Cargo,
    Npm,
    Dotnet,
    Maven,
    Python,
    Go,
    Gradle,
    CMake
}