use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct ContentString {
    pub value: String,
}

impl ContentString {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}

impl std::fmt::Display for ContentString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for ContentString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for ContentString {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for ContentString {}

impl From<&str> for ContentString {
    fn from(s: &str) -> Self {
        Self { value: s.to_string() }
    }
}

impl From<String> for ContentString {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ContentString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ContentStringVisitor;
        impl<'de> serde::de::Visitor<'de> for ContentStringVisitor {
            type Value = ContentString;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ContentString { value: v.to_string() })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(ContentString { value: v })
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
                Ok(ContentString { value: val })
            }
        }
        deserializer.deserialize_any(ContentStringVisitor)
    }
}
