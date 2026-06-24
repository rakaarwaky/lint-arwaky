// PURPOSE: Macros for generating boilerplate impls on String/primitive wrapper value objects.
//
// These macros emit the impls that every String-wrapper VO needs:
//   - `new(value)` constructor
//   - `value()` accessor
//   - `Display`
//   - `Hash` / `PartialEq` / `Eq` (optional)
//   - `From<&str>` / `From<String>` / `From<$Inner>` (for primitives)
//   - serde `Deserialize` (accepts either a primitive or a map with a `value` key)
//
// Using the macro keeps each VO file to its domain-specific surface and stops
// AES305 from flagging the same serde visitor across ~13 files.

/// Generate a String-wrapped value object with the standard VO surface.
///
/// # Usage
/// ```ignore
/// // in any sibling module file:
/// use crate::string_value_object;
/// string_value_object!(FooName);
/// ```
///
/// The macro is `#[macro_export]`-ed so it is accessible at the crate root.
/// Each VO file `use crate::string_value_object;` once and then invokes the
/// macro locally.
#[macro_export]
macro_rules! string_value_object {
    ($name:ident) => {
        #[derive(Default, Debug, Clone, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name {
            pub value: String,
        }

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self {
                    value: value.into(),
                }
            }

            pub fn value(&self) -> &str {
                &self.value
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.value.hash(state);
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl Eq for $name {}

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self {
                    value: s.to_string(),
                }
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self { value: s }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V {}
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = $name;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("primitive or map with 'value' key")
                    }
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name {
                            value: v.to_string(),
                        })
                    }
                    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v })
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
                        let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                        Ok($name { value: val })
                    }
                }
                deserializer.deserialize_any(V {})
            }
        }
    };
}

/// Generate a primitive-wrapped value object (e.g. `i64`, `f64`, `bool`).
///
/// # Usage
/// ```ignore
/// primitive_value_object!(LineNumber, i64);
/// ```
///
/// Emits the same surface as `string_value_object!` but with `From<$Inner>`,
/// `From<$Inner>` conversions, and a serde visitor that accepts the inner
/// type or a `{"value": ...}` map.
#[macro_export]
macro_rules! primitive_value_object {
    ($name:ident, $inner:ty) => {
        #[derive(Default, Debug, Clone, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name {
            pub value: $inner,
        }

        impl $name {
            pub fn new(value: $inner) -> Self {
                Self { value }
            }

            pub fn value(&self) -> $inner {
                self.value
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl Eq for $name {}

        impl From<$inner> for $name {
            fn from(v: $inner) -> Self {
                Self { value: v }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V {}
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = $name;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(concat!(
                            "primitive or map with 'value' key (",
                            stringify!($inner),
                            ")"
                        ))
                    }
                    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut value: Option<$inner> = None;
                        while let Some(k) = map.next_key::<String>()? {
                            if k == "value" {
                                value = Some(map.next_value::<$inner>()?);
                            } else {
                                let _: serde::de::IgnoredAny = map.next_value()?;
                            }
                        }
                        let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                        Ok($name { value: val })
                    }
                }
                deserializer.deserialize_any(V {})
            }
        }
    };
}

#[cfg(test)]
mod macro_tests {

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    string_value_object!(MyTestVO);

    #[test]
    fn string_vo_new_and_value() {
        let v = MyTestVO::new("hello");
        assert_eq!(v.value(), "hello");
        let v2 = MyTestVO::new(String::from("world"));
        assert_eq!(v2.value(), "world");
    }

    #[test]
    fn string_vo_default_is_empty() {
        let v = MyTestVO::default();
        assert_eq!(v.value(), "");
    }

    #[test]
    fn string_vo_display() {
        let v = MyTestVO::new("hello");
        assert_eq!(v.to_string(), "hello");
    }

    #[test]
    fn string_vo_from_str() {
        let v: MyTestVO = "abc".into();
        assert_eq!(v.value(), "abc");
    }

    #[test]
    fn string_vo_from_string() {
        let v: MyTestVO = String::from("xyz").into();
        assert_eq!(v.value(), "xyz");
    }

    #[test]
    fn string_vo_equality() {
        assert_eq!(MyTestVO::new("a"), MyTestVO::new("a"));
        assert_ne!(MyTestVO::new("a"), MyTestVO::new("b"));
    }

    #[test]
    fn string_vo_hash_matches_inner() {
        let v = MyTestVO::new("hashable");
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        v.hash(&mut h1);
        "hashable".hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn string_vo_deserialize_from_string() {
        let v: MyTestVO = serde_json::from_str("\"hello\"").unwrap();
        assert_eq!(v.value(), "hello");
    }

    #[test]
    fn string_vo_deserialize_from_map() {
        let v: MyTestVO = serde_json::from_str("{\"value\":\"wrapped\"}").unwrap();
        assert_eq!(v.value(), "wrapped");
    }

    #[test]
    fn string_vo_serialize_transparent() {
        let v = MyTestVO::new("plain");
        let s = serde_json::to_string(&v).unwrap();
        assert_eq!(s, "\"plain\"");
    }

    primitive_value_object!(MyNum, i64);

    #[test]
    fn primitive_vo_new_and_value() {
        let v = MyNum::new(42);
        assert_eq!(v.value(), 42);
    }

    #[test]
    fn primitive_vo_display() {
        let v = MyNum::new(123);
        assert_eq!(v.to_string(), "123");
    }

    #[test]
    fn primitive_vo_from_inner() {
        let v: MyNum = 7.into();
        assert_eq!(v.value(), 7);
    }

    #[test]
    fn primitive_vo_deserialize_from_int() {
        let v: MyNum = serde_json::from_str("99").unwrap();
        assert_eq!(v.value(), 99);
    }

    #[test]
    fn primitive_vo_deserialize_from_map() {
        let v: MyNum = serde_json::from_str("{\"value\":11}").unwrap();
        assert_eq!(v.value(), 11);
    }
}
