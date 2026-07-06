use serde::{Deserialize, Serialize};

use crate::{Severity, Target};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuleDefination {
    pub id: String,
    pub title: String,
    pub category: String,
    pub severity: Severity,
    pub target: Target,
    pub description: String,
    pub detection: String,
    pub recommendation: String,
}