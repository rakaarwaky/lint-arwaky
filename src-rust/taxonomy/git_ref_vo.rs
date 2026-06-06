use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct GitRef {
    pub value: String,
}

impl GitRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for GitRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for GitRef {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
    }
}

impl From<String> for GitRef {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for GitRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>,
    {
        struct GitRefVisitor;
        impl<'de> serde::de::Visitor<'de> for GitRefVisitor {
            type Value = GitRef;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(GitRef { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(GitRef { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" { value = Some(map.next_value::<String>()?); }
                    else { let _: serde::de::IgnoredAny = map.next_value()?; }
                }
                Ok(GitRef { value: value.unwrap_or_default() })
            }
        }
        deserializer.deserialize_any(GitRefVisitor)
    }
}
