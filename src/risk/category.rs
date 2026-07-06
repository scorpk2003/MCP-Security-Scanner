use serde::{Deserialize, Serialize};

use crate::Severity;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RiskCategory {
    SecretExposure,
    DangerousToolCapability,
    WeakInputSchema,
    AccessRisk(AccessCategory),
    CommandExecution,
    SensitiveLogging,
    OverPermissionTool,
    Other(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AccessCategory {
    Path,
    File,
    Network,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MCPRisk{
    pub rule_id: String,
    pub title: String,
    pub category: RiskCategory,
    pub evidence: Option<Evidence>,
    pub severity_default: Severity,
    pub recommendation: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Evidence {
    pub file: Option<String>,
    pub line: Option<usize>,
    pub snippet: Option<String>,
    pub tool_name: Option<String>,
}