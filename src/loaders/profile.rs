use core::panic;
use std::path::PathBuf;

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

impl Default for Profile {
    fn default() -> Self {
        let name = String::new();
        let root_path = None;
        let command = None;
        let runtime = McpRuntime::None;
        let config = Vec::new();
        let env = Vec::new();
        let source_files = Vec::new();
        let tools = Vec::new();

        Self { name, root_path, command, runtime, config, env, source_files, tools }
    }
}

impl Profile {
    pub fn loaders(request: &ScanRequest) -> Self {
        if let Some(max_level) = request.input_mode.iter().max() {
            match max_level {
                InputType::TARGET => {

                },
                InputType::COMMAND => {},
                InputType::CONFIG => {}
            }
        };
        panic!("\tCan't find input mode!!!");
    }
}

pub enum McpRuntime {
    Cargo,
    Node {runtime: String},
    Dotnet,
    Maven,
    Python,
    Go,
    Gradle,
    CMake,
    None
}

impl McpRuntime {
    pub fn parse_from_config_path(path: &str) -> McpRuntime {
        match path {
            "Cargo.toml" => McpRuntime::Cargo,
            "pyproject.toml" | "requirement.txt" => McpRuntime::Python,
            "package.json" => {
                let runtime = McpRuntime::detect_nodejs_runtime(path.clone());
                McpRuntime::Node { runtime }
            },
            "pom.xml" => McpRuntime::Maven,
            "build.gradle" => McpRuntime::Gradle,
            "CMakeList.txt" => McpRuntime::CMake,
            "go.mod" => McpRuntime::Go,
            _ => {
                if path.ends_with(".csproj") | path.ends_with(".sln") {
                    return McpRuntime::Dotnet
                }
                McpRuntime::None
            },
        }
    }
    fn detect_nodejs_runtime(path: &str) -> String {
        let path_buf = PathBuf::from(path);
        if let Some(parent) = path_buf.parent() {
            if parent.join("pnpm-lock.yaml").exists() {
                return "pnpm".to_string();
            } else if parent.join("yarn.lock").exists() {
                return "yarn".to_string();
            } else if parent.join("bun.lockb").exists() {
                return "bun".to_string();
            } else if parent.join("package-lock.json").exists() {
                return "npm".to_string();
            }
        }
        "npm (default)".to_string()
    }
}