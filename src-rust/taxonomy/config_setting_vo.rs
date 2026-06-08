//! config_setting_vo — Value objects for configuration domain.

use serde::{Deserialize, Serialize};

use crate::taxonomy::{
    AdapterName, ArchitectureConfig, Count, DescriptionVO, DirectoryPath, FilePathList,
    PatternList, Score,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ActualValue {
    pub(crate) value: String,
}

impl ActualValue {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for ActualValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ActualValue {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ActualValue {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ActualValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ActualValueVisitor {}
        impl<'de> serde::de::Visitor<'de> for ActualValueVisitor {
            type Value = ActualValue;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive string or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ActualValue {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ActualValue { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(ActualValue { value: val })
            }
        }
        deserializer.deserialize_any(ActualValueVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ExpectedValue {
    pub(crate) value: String,
}

impl ExpectedValue {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for ExpectedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ExpectedValue {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ExpectedValue {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ExpectedValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ExpectedValueVisitor {}
        impl<'de> serde::de::Visitor<'de> for ExpectedValueVisitor {
            type Value = ExpectedValue;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive string or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ExpectedValue {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ExpectedValue { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(ExpectedValue { value: val })
            }
        }
        deserializer.deserialize_any(ExpectedValueVisitor {})
    }
}

/// Scoring thresholds.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Thresholds {
    pub score: Score,
    pub complexity: Count,
    pub max_file_lines: Count,
}

impl Thresholds {
    pub fn new(score: Score, complexity: Count, max_file_lines: Count) -> Self {
        Self {
            score,
            complexity,
            max_file_lines,
        }
    }
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            score: Score::new(80.0),
            complexity: Count::new(10),
            max_file_lines: Count::new(500),
        }
    }
}

/// Adapter status enum.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum AdapterStatus {
    #[default]
    Enabled,
    Disabled,
    NotInstalled,
}

impl AdapterStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AdapterStatus::Enabled => "enabled",
            AdapterStatus::Disabled => "disabled",
            AdapterStatus::NotInstalled => "not_installed",
        }
    }
}

impl std::fmt::Display for AdapterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Single adapter configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterEntry {
    pub name: AdapterName,
    #[serde(default)]
    pub status: AdapterStatus,
    #[serde(default = "default_weight")]
    pub weight: f64,
}

fn default_weight() -> f64 {
    1.0
}

impl AdapterEntry {
    pub fn new(name: AdapterName, status: AdapterStatus, weight: f64) -> Self {
        Self {
            name,
            status,
            weight,
        }
    }

    pub fn enabled(name: AdapterName) -> Self {
        Self::new(name, AdapterStatus::Enabled, 1.0)
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, AdapterStatus::Enabled)
    }
}

/// Project configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectConfig {
    #[serde(default = "default_project_name")]
    pub project_name: DescriptionVO,
    #[serde(default)]
    pub thresholds: Thresholds,
    #[serde(default)]
    pub adapters: Vec<AdapterEntry>,
    #[serde(default)]
    pub ignored_paths: FilePathList,
    #[serde(default)]
    pub ignored_rules: PatternList,
    #[serde(default)]
    pub layer_map: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub output_dir: Option<DirectoryPath>,
    #[serde(default)]
    pub architecture: ArchitectureConfig,
}

fn default_project_name() -> DescriptionVO {
    DescriptionVO::new("lint-arwaky")
}

impl ProjectConfig {
    pub fn new(
        project_name: DescriptionVO,
        thresholds: Thresholds,
        adapters: Vec<AdapterEntry>,
        ignored_paths: FilePathList,
        ignored_rules: PatternList,
        layer_map: std::collections::HashMap<String, String>,
        output_dir: Option<DirectoryPath>,
        architecture: ArchitectureConfig,
    ) -> Self {
        Self {
            project_name,
            thresholds,
            adapters,
            ignored_paths,
            ignored_rules,
            layer_map,
            output_dir,
            architecture,
        }
    }

    /// Returns a ProjectConfig with default linter adapters enabled.
    pub fn defaults() -> Self {
        Self {
            project_name: default_project_name(),
            thresholds: Thresholds::default(),
            adapters: vec![
                AdapterEntry::enabled(AdapterName::raw("ruff")),
                AdapterEntry::enabled(AdapterName::raw("mypy")),
                AdapterEntry::enabled(AdapterName::raw("bandit")),
                AdapterEntry::enabled(AdapterName::raw("radon")),
            ],
            ignored_paths: FilePathList::default(),
            ignored_rules: PatternList::default(),
            layer_map: std::collections::HashMap::new(),
            output_dir: None,
            architecture: ArchitectureConfig::default(),
        }
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self::defaults()
    }
}
