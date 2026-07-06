use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Severity {
    CRITICAL,
    HIGH,
    MEDIUM,
    LOW,
    INFO,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Target {
    SOURCE,
    CONFIG,
    ENV,
    TOOLSCHEMA,
    OTHER(String)
}