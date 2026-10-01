// PURPOSE: Graph-color VO for DFS cycle detection; DependencyEdge and ResolvedImport.
use serde::{Deserialize, Serialize};

use shared_common::taxonomy_name_vo::SymbolName;

/// Graph traversal colour used by the cycle-import checker's DFS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphColorVO {
    #[default]
    White,
    Gray,
    Black,
}

/// Directed dependency edge between two layer identifiers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DependencyEdge {
    pub source: String,
    pub target: String,
}

impl DependencyEdge {
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
}

/// Result of resolving an import through a barrel file.
///
/// When an import goes through a barrel file (__init__.py, index.ts, mod.rs),
/// the original module path hides the source file name and its layer prefix.
/// This VO carries the resolution result so checkers can detect the correct layer.
///
/// # Example
/// ```text
/// import:   from modules.shared.src.server import IBlenderConnectionProtocol
/// barrel:   modules/shared/src/server/__init__.py
///           → from .contract_connection_protocol import IBlenderConnectionProtocol
/// resolved: ResolvedImport {
///     original_module: "modules.shared.src.server",
///     resolved_file:   "contract_connection_protocol",
///     resolved_layer:  Some("contract"),
///     symbol:          "IBlenderConnectionProtocol",
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedImport {
    /// Original module path as written in the import statement.
    pub original_module: String,
    /// Resolved source file stem (filename without extension).
    pub resolved_file: String,
    /// Detected architectural layer from the resolved file name.
    pub resolved_layer: Option<String>,
    /// The symbol that was imported.
    pub symbol: String,
}

impl ResolvedImport {
    pub fn new(
        original_module: impl Into<String>,
        resolved_file: impl Into<String>,
        resolved_layer: Option<String>,
        symbol: impl Into<String>,
    ) -> Self {
        Self {
            original_module: original_module.into(),
            resolved_file: resolved_file.into(),
            resolved_layer,
            symbol: symbol.into(),
        }
    }

    /// Check if the resolved layer matches the expected layer.
    pub fn matches_layer(&self, expected: &str) -> bool {
        self.resolved_layer.as_deref() == Some(expected)
    }

    /// Check if the resolved file name contains the given suffix.
    pub fn has_suffix(&self, suffix: &str) -> bool {
        self.resolved_file
            .to_lowercase()
            .contains(&format!("_{}", suffix.to_lowercase()))
    }
}

/// Import-rule violation (AES201-AES205) — payload emitted by capability checkers.
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_message_vo::LintMessage;

#[derive(Debug, Clone)]
pub enum AesImportViolation {
    ForbiddenImport {
        source_layer: LayerNameVO,
        forbidden_layer: LayerNameVO,
        allowed: Vec<LayerNameVO>,
        reason: Option<LintMessage>,
        fix: LintMessage,
    },
    MissingImport {
        source_layer: LayerNameVO,
        required: SymbolName,
        reason: Option<LintMessage>,
    },
    FixUnusedImport {
        reason: Option<LintMessage>,
    },
    ImportIntentViolation {
        source_layer: LayerNameVO,
        import_type: SymbolName,
        intent: SymbolName,
        reason: Option<LintMessage>,
    },
    CircularImport {
        reason: Option<LintMessage>,
    },
}
