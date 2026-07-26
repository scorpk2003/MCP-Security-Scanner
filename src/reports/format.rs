use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{MCPRisk};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReportFormat {
    pub target: TargetReport,
    pub summary: Map<String, Value>,
    pub findings: Vec<MCPRisk>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetReport {
    pub name: TargetType,
    pub runtime: String,
    pub tools_found: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TargetType {
    JSON,
    MARKDOWN,
    CONSOLE,
}