use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct ConfigKey {
    pub(crate) value: String,
}

impl ConfigKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn parts(&self) -> Vec<String> {
        self.value.split('.').map(|s| s.to_string()).collect()
    }
    pub fn parent(&self) -> String {
        let parts = self.parts();
        if parts.len() > 1 {
            parts[..parts.len() - 1].join(".")
        } else {
            String::new()
        }
    }
    pub fn leaf(&self) -> String {
        self.parts()
            .last()
            .cloned()
            .unwrap_or_else(|| self.value.clone())
    }
}

impl std::fmt::Display for ConfigKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for ConfigKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for ConfigKey {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for ConfigKey {}

impl From<&str> for ConfigKey {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ConfigKey {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl Default for ConfigKey {
    fn default() -> Self {
        ConfigKey {
            value: String::new(),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ConfigKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ConfigKeyVisitor;
        impl<'de> serde::de::Visitor<'de> for ConfigKeyVisitor {
            type Value = ConfigKey;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ConfigKey {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ConfigKey { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" || k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(ConfigKey { value: val })
            }
        }
        deserializer.deserialize_any(ConfigKeyVisitor)
    }
}
