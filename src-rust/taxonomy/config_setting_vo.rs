use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use super::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct ActualValue {
    pub value: String,
}

impl ActualValue {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for ActualValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ActualValue {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
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
        struct ActualValueVisitor;
        impl<'de> serde::de::Visitor<'de> for ActualValueVisitor {
            type Value = ActualValue;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ActualValue { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ActualValue { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" || k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _ : serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(ActualValue { value: val })
            }
        }
        deserializer.deserialize_any(ActualValueVisitor)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct ExpectedValue {
    pub value: String,
}

impl ExpectedValue {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for ExpectedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ExpectedValue {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
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
        struct ExpectedValueVisitor;
        impl<'de> serde::de::Visitor<'de> for ExpectedValueVisitor {
            type Value = ExpectedValue;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ExpectedValue { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ExpectedValue { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" || k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _ : serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(ExpectedValue { value: val })
            }
        }
        deserializer.deserialize_any(ExpectedValueVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Thresholds {
    pub score: Score,
    pub complexity: Count,
    pub max_file_lines: Count,
}

impl Thresholds {
    pub fn new(score: Score, complexity: Count, max_file_lines: Count,) -> Self {
        Self { score, complexity, max_file_lines }
    }
}
