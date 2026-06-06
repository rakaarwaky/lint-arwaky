use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct ClassPath {
    pub value: String,
}

impl ClassPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for ClassPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ClassPath {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
    }
}

impl From<String> for ClassPath {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ClassPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ClassPathVisitor;
        impl<'de> serde::de::Visitor<'de> for ClassPathVisitor {
            type Value = ClassPath;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ClassPath { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ClassPath { value: v })
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
                Ok(ClassPath { value: val })
            }
        }
        deserializer.deserialize_any(ClassPathVisitor)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct DescriptionVO {
    pub value: String,
}

impl DescriptionVO {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for DescriptionVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for DescriptionVO {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
    }
}

impl From<String> for DescriptionVO {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for DescriptionVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DescriptionVOVisitor;
        impl<'de> serde::de::Visitor<'de> for DescriptionVOVisitor {
            type Value = DescriptionVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(DescriptionVO { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(DescriptionVO { value: v })
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
                Ok(DescriptionVO { value: val })
            }
        }
        deserializer.deserialize_any(DescriptionVOVisitor)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct LogOutput {
    pub value: String,
}

impl LogOutput {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for LogOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for LogOutput {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
    }
}

impl From<String> for LogOutput {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for LogOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LogOutputVisitor;
        impl<'de> serde::de::Visitor<'de> for LogOutputVisitor {
            type Value = LogOutput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(LogOutput { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(LogOutput { value: v })
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
                Ok(LogOutput { value: val })
            }
        }
        deserializer.deserialize_any(LogOutputVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetadataVO {
    pub value: std::collections::HashMap<String, serde_json::Value>,
}

impl MetadataVO {
    pub fn new(value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self { value: value }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct StdError {
    pub value: String,
}

impl StdError {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for StdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for StdError {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
    }
}

impl From<String> for StdError {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for StdError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StdErrorVisitor;
        impl<'de> serde::de::Visitor<'de> for StdErrorVisitor {
            type Value = StdError;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(StdError { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(StdError { value: v })
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
                Ok(StdError { value: val })
            }
        }
        deserializer.deserialize_any(StdErrorVisitor)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct StdOutput {
    pub value: String,
}

impl StdOutput {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for StdOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for StdOutput {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
    }
}

impl From<String> for StdOutput {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for StdOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StdOutputVisitor;
        impl<'de> serde::de::Visitor<'de> for StdOutputVisitor {
            type Value = StdOutput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(StdOutput { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(StdOutput { value: v })
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
                Ok(StdOutput { value: val })
            }
        }
        deserializer.deserialize_any(StdOutputVisitor)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Suggestion {
    pub value: String,
}

impl Suggestion {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for Suggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for Suggestion {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
    }
}

impl From<String> for Suggestion {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for Suggestion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SuggestionVisitor;
        impl<'de> serde::de::Visitor<'de> for SuggestionVisitor {
            type Value = Suggestion;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(Suggestion { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(Suggestion { value: v })
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
                Ok(Suggestion { value: val })
            }
        }
        deserializer.deserialize_any(SuggestionVisitor)
    }
}
