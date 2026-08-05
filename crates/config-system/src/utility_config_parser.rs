/// Parse the score threshold from raw YAML config content.
/// Checks `project.thresholds.score` then `thresholds.score`.
/// Returns the value if found, otherwise None.
pub fn parse_score_threshold(yaml_str: &str) -> Option<f64> {
    let raw: serde_yaml_ng::Value = serde_yaml_ng::from_str(yaml_str).ok()?;
    raw.get("project")
        .and_then(|p| p.get("thresholds"))
        .and_then(|t| t.get("score"))
        .and_then(|s| s.as_f64())
        .or_else(|| {
            raw.get("thresholds")
                .and_then(|t| t.get("score"))
                .and_then(|s| s.as_f64())
        })
}

/// Returns names of adapters whose status is "enabled" (default).
/// Ignores special entries like "architecture" (internal analysis).
/// Supports both `status` (string) and `enabled` (bool) fields per Appendix A.
pub fn parse_adapter_names_from_yaml(yaml_str: &str) -> Vec<String> {
    let raw: serde_yaml_ng::Value = match serde_yaml_ng::from_str(yaml_str) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let Some(adapters) = raw.get("adapters").and_then(|a| a.as_sequence()) else {
        return Vec::new();
    };
    adapters
        .iter()
        .filter_map(|entry| {
            let name = entry.get("name")?.as_str()?;
            if name == "architecture" {
                return None;
            }
            // Support both "enabled" (bool) and "status" (string) fields
            let enabled = entry
                .get("enabled")
                .and_then(|v| v.as_bool())
                .or_else(|| {
                    entry
                        .get("status")
                        .and_then(|s| s.as_str())
                        .map(|s| s != "disabled")
                })
                .unwrap_or(true);
            if enabled {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Parse adapter entries from YAML, returning `Vec<AdapterEntry>` with name,
/// status, weight, and timeout fields. Skips disabled adapters.
///
/// Supports the Appendix A YAML schema:
/// ```yaml
/// adapters:
///   - name: "clippy"
///     enabled: true
///     weight: 1.0
///     timeout: 120
/// ```
pub fn parse_adapter_entries_from_yaml(
    yaml_str: &str,
) -> Vec<shared::config_system::taxonomy_setting_vo::AdapterEntry> {
    use shared::common::taxonomy_adapter_name_vo::AdapterName;
    use shared::config_system::taxonomy_setting_vo::{AdapterEntry, AdapterStatus};
    let raw: serde_yaml_ng::Value = match serde_yaml_ng::from_str(yaml_str) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let Some(adapters) = raw.get("adapters").and_then(|a| a.as_sequence()) else {
        return Vec::new();
    };
    adapters
        .iter()
        .filter_map(|entry| {
            let name = entry.get("name")?.as_str()?;
            if name == "architecture" {
                return None;
            }
            // Support both "status" (existing) and "enabled" (FRD Appendix A) fields
            let enabled = entry
                .get("enabled")
                .and_then(|v| v.as_bool())
                .or_else(|| {
                    entry
                        .get("status")
                        .and_then(|s| s.as_str())
                        .map(|s| s != "disabled")
                })
                .unwrap_or(true);
            if !enabled {
                return None;
            }
            let weight = entry.get("weight").and_then(|v| v.as_f64()).unwrap_or(1.0);
            let timeout = entry
                .get("timeout")
                .and_then(|v| v.as_f64())
                .unwrap_or(60.0);
            Some(AdapterEntry::with_timeout(
                AdapterName::raw(name.to_string()),
                AdapterStatus::Enabled,
                weight,
                timeout,
            ))
        })
        .collect()
}
