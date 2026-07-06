use crate::MCPTool;


pub struct Profile {
    pub name: String,
    pub root_path: Option<String>,
    pub command: Option<String>,
    pub runtime: String,
    pub config: Vec<String>,
    pub env: Vec<String>,
    pub source_files: Vec<String>,
    pub tools: Vec<MCPTool>,
}