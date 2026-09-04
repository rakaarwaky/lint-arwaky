// PURPOSE: Taxonomy Value Objects for embedded skills and installation filtering
use serde::{Deserialize, Serialize};

/// Represents an embedded skill file compiled directly into the binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedSkillVO {
    pub name: &'static str,
    pub relative_path: &'static str,
    pub content: &'static str,
    pub language: Option<&'static str>,
}

impl EmbeddedSkillVO {
    pub const fn new(
        name: &'static str,
        relative_path: &'static str,
        content: &'static str,
        language: Option<&'static str>,
    ) -> Self {
        Self {
            name,
            relative_path,
            content,
            language,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn relative_path(&self) -> &'static str {
        self.relative_path
    }

    pub const fn content(&self) -> &'static str {
        self.content
    }

    pub const fn language(&self) -> Option<&'static str> {
        self.language
    }
}
