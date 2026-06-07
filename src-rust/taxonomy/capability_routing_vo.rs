use serde::{Deserialize, Serialize};

use crate::taxonomy::{DescriptionVO, FilePath, LineNumber};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapabilityReference {
    pub file: FilePath,
    pub line: LineNumber,
    pub class_name: ClassNameVO,
    pub method_name: DescriptionVO,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapabilityReferenceList {
    pub references: Vec<CapabilityReference>,
}

impl CapabilityReferenceList {
    pub fn new(value: Vec<CapabilityReference>) -> Self {
        Self { references: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapabilityRoutingContext {
    pub references: CapabilityReferenceList,
    pub definitions: ClassDefinitionMap,
    pub files: ClassFileMap,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassDefinitionMap {
    pub definitions: std::collections::HashMap<ClassNameVO, ClassMethodsVO>,
}

impl ClassDefinitionMap {
    pub fn new(value: std::collections::HashMap<ClassNameVO, ClassMethodsVO>) -> Self {
        Self { definitions: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassFileMap {
    pub mapping: std::collections::HashMap<ClassNameVO, FilePath>,
}

impl ClassFileMap {
    pub fn new(value: std::collections::HashMap<ClassNameVO, FilePath>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassMethodsVO {
    pub methods: Vec<DescriptionVO>,
}

impl ClassMethodsVO {
    pub fn new(value: Vec<DescriptionVO>) -> Self {
        Self { methods: value }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ClassNameVO {
    pub(crate) value: String,
}

impl ClassNameVO {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for ClassNameVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ClassNameVO {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ClassNameVO {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ClassNameVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ClassNameVOVisitor;
        impl<'de> serde::de::Visitor<'de> for ClassNameVOVisitor {
            type Value = ClassNameVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ClassNameVO {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ClassNameVO { value: v })
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
                Ok(ClassNameVO { value: val })
            }
        }
        deserializer.deserialize_any(ClassNameVOVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassUsageItem {
    pub file: FilePath,
    pub line: LineNumber,
    pub method: DescriptionVO,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassUsageItemList {
    pub items: Vec<ClassUsageItem>,
}

impl ClassUsageItemList {
    pub fn new(value: Vec<ClassUsageItem>) -> Self {
        Self { items: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassUsageMap {
    pub usage: std::collections::HashMap<ClassNameVO, ClassUsageItemList>,
}

impl ClassUsageMap {
    pub fn new(value: std::collections::HashMap<ClassNameVO, ClassUsageItemList>) -> Self {
        Self { usage: value }
    }
}

impl CapabilityRoutingContext {
    pub fn new(
        references: CapabilityReferenceList,
        definitions: ClassDefinitionMap,
        files: ClassFileMap,
    ) -> Self {
        Self {
            references,
            definitions,
            files,
        }
    }
}

impl CapabilityReference {
    pub fn new(
        file: FilePath,
        line: LineNumber,
        class_name: ClassNameVO,
        method_name: DescriptionVO,
    ) -> Self {
        Self {
            file,
            line,
            class_name,
            method_name,
        }
    }
}

impl ClassUsageItem {
    pub fn new(file: FilePath, line: LineNumber, method: DescriptionVO) -> Self {
        Self { file, line, method }
    }
}
