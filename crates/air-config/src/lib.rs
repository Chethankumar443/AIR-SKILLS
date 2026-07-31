//! Configuration files (.air/config.json, .air/lock.json) schemas and schema migration logic.

use serde::{Deserialize, Serialize};

pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Structure of `.air/config.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirConfig {
    pub schema_version: u32,
    pub github_token: Option<String>,
    pub registry_url: String,
    pub telemetry_enabled: bool,
    pub strict_merge: bool,
}

impl Default for AirConfig {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            github_token: None,
            registry_url: "https://github.com/AIR-SKILLS".to_string(),
            telemetry_enabled: false,
            strict_merge: false,
        }
    }
}

/// Migration manager for updating on-disk configuration schemas
pub struct ConfigMigrator;

impl ConfigMigrator {
    pub fn migrate_if_needed(raw_json: &str) -> Result<AirConfig, String> {
        let parsed: serde_json::Value = serde_json::from_str(raw_json)
            .map_err(|e| format!("Failed to parse config JSON: {e}"))?;

        let version = parsed.get("schema_version").and_then(|v| v.as_u64()).unwrap_or(1) as u32;

        if version > CURRENT_SCHEMA_VERSION {
            return Err(format!(
                "Config schema version {} is newer than supported version {}",
                version, CURRENT_SCHEMA_VERSION
            ));
        }

        let config: AirConfig = serde_json::from_value(parsed)
            .map_err(|e| format!("Failed to map JSON to AirConfig: {e}"))?;

        Ok(config)
    }
}
