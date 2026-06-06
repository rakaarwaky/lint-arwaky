use serde::{Serialize, Deserialize};

use crate::taxonomy::MetadataVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionArgs {
    pub value: MetadataVO,
}

impl ActionArgs {
    pub fn new(value: MetadataVO) -> Self {
        Self { value: value }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct ActionName {
    pub value: String,
}

impl ActionName {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for ActionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for ActionName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for ActionName {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for ActionName {}

impl From<&str> for ActionName {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
    }
}

impl From<String> for ActionName {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ActionName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ActionNameVisitor;
        impl<'de> serde::de::Visitor<'de> for ActionNameVisitor {
            type Value = ActionName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ActionName { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ActionName { value: v })
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
                Ok(ActionName { value: val })
            }
        }
        deserializer.deserialize_any(ActionNameVisitor)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct JobId {
    pub value: String,
}

impl JobId {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for JobId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for JobId {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for JobId {}

impl From<&str> for JobId {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
    }
}

impl From<String> for JobId {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for JobId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct JobIdVisitor;
        impl<'de> serde::de::Visitor<'de> for JobIdVisitor {
            type Value = JobId;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(JobId { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(JobId { value: v })
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
                Ok(JobId { value: val })
            }
        }
        deserializer.deserialize_any(JobIdVisitor)
    }
}
