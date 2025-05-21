//! Nexus configuration for Magnet.toml files

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Nexus-specific configuration
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NexusConfig {
    /// Name of the nexus
    pub name: Option<String>,
    /// Version of the nexus
    pub version: Option<String>,
    pub members: Vec<String>,
    pub exclude: Vec<String>,
    /// Description of the nexus
    #[serde(default)]
    pub description: Option<String>,

    /// Custom nexus metadata
    #[serde(flatten)]
    pub custom: HashMap<String, toml::Value>,
}