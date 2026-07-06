use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{MCPRisk, Severity};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReportFormat {
    pub target: TargetReport,
    pub summary: Map<String, Value>,
    pub findings: Vec<MCPRisk>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetReport {
    pub name: String,
    pub runtime: String,
    pub tools_found: usize,
}