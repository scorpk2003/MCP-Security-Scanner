use crate::Severity;



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

pub enum AccessCategory {
    Path,
    File,
    Network,
}

pub struct MCPRisk{
    pub category: RiskCategory,
    pub detection_idea: String,
    pub evidence: Option<Evidence>,
    pub severity_default: Severity,
    pub recommendation: String,
}

pub struct Evidence {
    pub file: Option<String>,
    pub line: Option<usize>,
    pub snippet: Option<String>,
    pub tool_name: Option<String>,
}