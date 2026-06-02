use serde::{Serialize, Deserialize};
use super::*;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct LineNumber {
    pub value: i64,
}

impl LineNumber {
    pub fn new(value: i64) -> Self {
        Self { value: value.max(0) }
    }
}

impl std::fmt::Display for LineNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for LineNumber {
    fn from(v: i64) -> Self { Self::new(v) }
}

impl From<u32> for LineNumber {
    fn from(v: u32) -> Self { Self::new(v as i64) }
}

impl<'de> serde::Deserialize<'de> for LineNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>,
    {
        struct LineNumberVisitor;
        impl<'de> serde::de::Visitor<'de> for LineNumberVisitor {
            type Value = LineNumber;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("integer or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(LineNumber { value: v.max(0) })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(LineNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" { value = Some(map.next_value::<i64>()?); }
                    else { let _: serde::de::IgnoredAny = map.next_value()?; }
                }
                Ok(LineNumber { value: value.unwrap_or(0).max(0) })
            }
        }
        deserializer.deserialize_any(LineNumberVisitor)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct ColumnNumber {
    pub value: i64,
}

impl ColumnNumber {
    pub fn new(value: i64) -> Self {
        Self { value: value.max(0) }
    }
}

impl std::fmt::Display for ColumnNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ColumnNumber {
    fn from(v: i64) -> Self { Self::new(v) }
}

impl From<u32> for ColumnNumber {
    fn from(v: u32) -> Self { Self::new(v as i64) }
}

impl<'de> serde::Deserialize<'de> for ColumnNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>,
    {
        struct ColumnNumberVisitor;
        impl<'de> serde::de::Visitor<'de> for ColumnNumberVisitor {
            type Value = ColumnNumber;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("integer or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ColumnNumber { value: v.max(0) })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ColumnNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" { value = Some(map.next_value::<i64>()?); }
                    else { let _: serde::de::IgnoredAny = map.next_value()?; }
                }
                Ok(ColumnNumber { value: value.unwrap_or(0).max(0) })
            }
        }
        deserializer.deserialize_any(ColumnNumberVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub line: LineNumber,
    #[serde(default)]
    pub column: ColumnNumber,
}

impl Position {
    pub fn new(line: LineNumber) -> Self {
        Self { line, column: ColumnNumber::new(0) }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.column.value > 0 {
            write!(f, "{}:{}", self.line, self.column)
        } else {
            write!(f, "{}", self.line)
        }
    }
}
