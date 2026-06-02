use serde::{Serialize, Deserialize};
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PluginGroup {
    pub value: String,
}

impl PluginGroup {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for PluginGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for PluginGroup {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
    }
}

impl From<String> for PluginGroup {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for PluginGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>,
    {
        struct PluginGroupVisitor;
        impl<'de> serde::de::Visitor<'de> for PluginGroupVisitor {
            type Value = PluginGroup;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(PluginGroup { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(PluginGroup { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" { value = Some(map.next_value::<String>()?); }
                    else { let _: serde::de::IgnoredAny = map.next_value()?; }
                }
                Ok(PluginGroup { value: value.unwrap_or_default() })
            }
        }
        deserializer.deserialize_any(PluginGroupVisitor)
    }
}
