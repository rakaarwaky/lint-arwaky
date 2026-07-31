// PURPOSE: taxonomy_orphan_parse_result_vo — value objects for AST/structured parse results.
// Shared across orphan-detector and import-rules. All parsers return these types.

use serde::{Deserialize, Serialize};

// ─── Block 1: Import Edge ─────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstImportVO {
    pub raw_path: String,
    pub segments: Vec<String>,
    pub is_reexport: bool,
    pub is_glob: bool,
    /// Line number (1-based) where this import appears
    pub line: usize,
}

impl AstImportVO {
    pub fn new(
        raw_path: String,
        segments: Vec<String>,
        is_reexport: bool,
        is_glob: bool,
        line: usize,
    ) -> Self {
        Self {
            raw_path,
            segments,
            is_reexport,
            is_glob,
            line,
        }
    }

    /// Get the last segment (typically the imported symbol name).
    pub fn last_segment(&self) -> Option<&str> {
        self.segments.last().map(|s| s.as_str())
    }

    /// Get the module path (all segments except the last).
    pub fn module_path(&self) -> String {
        if self.segments.len() <= 1 {
            return self.raw_path.clone();
        }
        self.segments[..self.segments.len() - 1].join("::")
    }
}

// ─── Block 2: Trait Implementation ────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstTraitImplVO {
    pub trait_name: String,
    pub type_name: String,
    pub has_generics: bool,
    /// Line number (1-based)
    pub line: usize,
    /// Whether ALL method bodies are empty/todo/unimplemented
    pub is_dummy: bool,
}

// ─── Block 3: Definitions ─────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstStructDefVO {
    pub name: String,
    pub is_pub: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstTraitDefVO {
    pub name: String,
    pub is_pub: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstModDeclVO {
    pub name: String,
    pub path_attr: Option<String>,
    pub is_pub: bool,
}

// ─── Block 4: Function Definition ─────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstFnDefVO {
    pub name: String,
    pub is_pub: bool,
    /// Line number (1-based)
    pub line: usize,
    /// End line number (1-based)
    pub end_line: usize,
    /// Whether the function body is empty or only contains todo!/unimplemented!/panic!
    pub is_dummy: bool,
}

// ─── Block 5: Per-Language Parse Results ──────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RustParseResultVO {
    pub imports: Vec<AstImportVO>,
    pub trait_impls: Vec<AstTraitImplVO>,
    pub structs: Vec<AstStructDefVO>,
    pub traits: Vec<AstTraitDefVO>,
    pub mod_decls: Vec<AstModDeclVO>,
    pub functions: Vec<AstFnDefVO>,
    /// All identifier references in the file body (for usage tracking)
    pub used_identifiers: Vec<String>,
    pub parse_ok: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PythonParseResultVO {
    pub imports: Vec<AstImportVO>,
    pub class_bases: Vec<(String, Vec<String>)>,
    pub functions: Vec<AstFnDefVO>,
    pub used_identifiers: Vec<String>,
    pub parse_ok: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TsParseResultVO {
    pub imports: Vec<AstImportVO>,
    pub class_implements: Vec<(String, Vec<String>)>,
    pub functions: Vec<AstFnDefVO>,
    pub used_identifiers: Vec<String>,
    pub parse_ok: bool,
}

// ─── Block 6: Unified Result ──────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileParseResultVO {
    Rust(RustParseResultVO),
    Python(PythonParseResultVO),
    TypeScript(TsParseResultVO),
    Unsupported,
}

// ─── Block 7: Query Helpers ───────────────────────────────

impl RustParseResultVO {
    pub fn has_trait_impl(&self, trait_name: &str) -> bool {
        self.trait_impls.iter().any(|ti| {
            ti.trait_name == trait_name || ti.trait_name.ends_with(&format!("::{}", trait_name))
        })
    }

    pub fn trait_names(&self) -> Vec<String> {
        self.traits.iter().map(|t| t.name.clone()).collect()
    }

    pub fn struct_names(&self) -> Vec<String> {
        self.structs.iter().map(|s| s.name.clone()).collect()
    }

    pub fn aggregate_trait_names(&self) -> Vec<String> {
        self.trait_impls
            .iter()
            .filter(|ti| ti.trait_name.contains("Aggregate"))
            .map(|ti| ti.trait_name.clone())
            .collect()
    }

    /// Check if an identifier is used anywhere in the file body.
    pub fn is_identifier_used(&self, name: &str) -> bool {
        self.used_identifiers.iter().any(|id| id == name)
    }

    /// Get all dummy trait implementations.
    pub fn dummy_trait_impls(&self) -> Vec<&AstTraitImplVO> {
        self.trait_impls.iter().filter(|ti| ti.is_dummy).collect()
    }

    /// Get all dummy functions.
    pub fn dummy_functions(&self) -> Vec<&AstFnDefVO> {
        self.functions.iter().filter(|f| f.is_dummy).collect()
    }
}

impl PythonParseResultVO {
    pub fn class_names(&self) -> Vec<String> {
        self.class_bases
            .iter()
            .map(|(name, _)| name.clone())
            .collect()
    }

    pub fn aggregate_names(&self) -> Vec<String> {
        self.class_bases
            .iter()
            .flat_map(|(_, bases)| bases.clone())
            .filter(|name| name.contains("Aggregate"))
            .collect()
    }

    pub fn is_identifier_used(&self, name: &str) -> bool {
        self.used_identifiers.iter().any(|id| id == name)
    }
}

impl TsParseResultVO {
    pub fn class_names(&self) -> Vec<String> {
        self.class_implements
            .iter()
            .map(|(name, _)| name.clone())
            .collect()
    }

    pub fn aggregate_names(&self) -> Vec<String> {
        self.class_implements
            .iter()
            .flat_map(|(_, ifaces)| ifaces.clone())
            .filter(|name| name.contains("Aggregate"))
            .collect()
    }

    pub fn is_identifier_used(&self, name: &str) -> bool {
        self.used_identifiers.iter().any(|id| id == name)
    }
}
