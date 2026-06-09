use serde::Serialize;

use crate::output_report::taxonomy_result_vo::LintResult;

pub fn compute_score(results: &[LintResult]) -> f64 {
    let penalty: f64 = results.iter().map(|r| r.severity.score_impact()).sum();
    (100.0 - penalty).max(0.0)
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct FileFormat {
    pub(crate) name: String,
}

impl FileFormat {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self { name: value.into() }
    }
    pub fn is_structured(&self) -> bool {
        matches!(self.name.as_ref(), "json" | "sarif" | "junit")
    }
}

impl std::fmt::Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&str> for FileFormat {
    fn from(s: &str) -> Self {
        Self {
            name: s.to_string(),
        }
    }
}

impl From<String> for FileFormat {
    fn from(s: String) -> Self {
        Self { name: s }
    }
}

impl<'de> serde::Deserialize<'de> for FileFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FileFormatVisitor {}
        impl<'de> serde::de::Visitor<'de> for FileFormatVisitor {
            type Value = FileFormat;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FileFormat {
                    name: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FileFormat { name: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "name" || k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                Ok(FileFormat { name: val })
            }
        }
        deserializer.deserialize_any(FileFormatVisitor {})
    }
}
