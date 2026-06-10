// PURPOSE: VO: Suffix value object
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct SuffixVO {
    pub(crate) value: String,
}

impl SuffixVO {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for SuffixVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for SuffixVO {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl<'de> serde::Deserialize<'de> for SuffixVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SuffixVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for SuffixVOVisitor {
            type Value = SuffixVO;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SuffixVO {
                    value: v.to_string(),
                })
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
                Ok(SuffixVO {
                    value: value.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_any(SuffixVOVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct SuffixPolicyVO {
    pub(crate) value: String,
}

impl SuffixPolicyVO {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn is_flexible(&self) -> bool {
        self.value == "flexible"
    }
    pub fn is_strict(&self) -> bool {
        self.value == "strict"
    }
}

impl Default for SuffixPolicyVO {
    fn default() -> Self {
        SuffixPolicyVO {
            value: "strict".to_string(),
        }
    }
}

impl std::fmt::Display for SuffixPolicyVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for SuffixPolicyVO {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl<'de> serde::Deserialize<'de> for SuffixPolicyVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SuffixPolicyVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for SuffixPolicyVOVisitor {
            type Value = SuffixPolicyVO;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SuffixPolicyVO {
                    value: v.to_string(),
                })
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
                Ok(SuffixPolicyVO {
                    value: value.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_any(SuffixPolicyVOVisitor {})
    }
}
