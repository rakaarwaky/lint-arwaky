// PURPOSE: Duration, Timeout — value objects for duration and timeout tracking
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Duration {
    pub(crate) value: f64,
}

impl Duration {
    pub fn new(value: f64) -> Self {
        Self {
            value: value.max(0.0),
        }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}ms", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DurationVisitor {}
        impl<'de> serde::de::Visitor<'de> for DurationVisitor {
            type Value = Duration;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("number or map with 'value' key")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Duration { value: v.max(0.0) })
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Duration {
                    value: (v as f64).max(0.0),
                })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<f64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                Ok(Duration {
                    value: value.unwrap_or(0.0).max(0.0),
                })
            }
        }
        deserializer.deserialize_any(DurationVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Timeout {
    pub(crate) value: f64,
}

impl Timeout {
    pub fn new(value: f64) -> Self {
        Self {
            value: value.max(0.001),
        }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl std::fmt::Display for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for Timeout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimeoutVisitor {}
        impl<'de> serde::de::Visitor<'de> for TimeoutVisitor {
            type Value = Timeout;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("number or map with 'value' key")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timeout {
                    value: v.max(0.001),
                })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<f64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                Ok(Timeout {
                    value: value.unwrap_or(30.0),
                })
            }
        }
        deserializer.deserialize_any(TimeoutVisitor {})
    }
}
