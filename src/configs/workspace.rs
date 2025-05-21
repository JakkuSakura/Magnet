//! Workspace configuration for Magnet.toml files

use crate::configs::DependencyConfigMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Workspace configuration (legacy)
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkspaceConfig {
    /// Workspace members (glob patterns)
    #[serde(default)]
    pub members: Vec<String>,
    /// Excluded workspace members (glob patterns)
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Cargo resolver version (1 or 2)
    #[serde(default)]
    pub resolver: Option<String>,
    #[serde(default, skip_serializing_if = "DependencyConfigMap::is_empty")]
    pub dependencies: DependencyConfigMap,
    /// Custom workspace metadata
    #[serde(flatten)]
    pub custom: HashMap<String, toml::Value>,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            members: Vec::new(),
            exclude: Vec::new(),
            resolver: None,
            dependencies: DependencyConfigMap::new(),
            custom: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CargoWorkspaceConfig {
    /// Workspace members (glob patterns)
    #[serde(default)]
    pub members: Vec<String>,
    /// Excluded workspace members (glob patterns)
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Cargo resolver version (1 or 2)
    #[serde(default)]
    pub resolver: Option<String>,
    
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub dependencies: DependencyConfigMap,
    
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CargoWorkspaceConfigWrapper {
    pub workspace: CargoWorkspaceConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<toml::value::Table>,
}