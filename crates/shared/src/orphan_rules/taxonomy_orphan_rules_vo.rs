// PURPOSE: Orphan rule value objects — contract list VOs, parse-result types,
//          and the violation enum (AES501-506).
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use shared_common::taxonomy_message_vo::LintMessage;
use shared_common::taxonomy_path_vo::FilePath;

// ─── Contract surface VOs ──────────────────────────────────────────────

/// List of file paths under orphan-detection analysis. Wraps `Vec<String>`
/// (raw path strings as emitted by the directory walker). Replaces the
/// previous `&[String]` parameter and `Vec<String>` return type used in
/// `IOrphanGraphResolverProtocol::build_graph_context` and
/// `identify_entry_points`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrphanFileListVO {
    pub values: Vec<String>,
}

impl OrphanFileListVO {
    pub fn new(values: Vec<String>) -> Self {
        Self { values }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// List of configured entry-point patterns (e.g. glob prefixes or exact
/// paths) the resolver should treat as reachable entry points. Replaces
/// the previous `&[String]` parameter on
/// `IOrphanGraphResolverProtocol::identify_entry_points`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrphanEntryPatternListVO {
    pub values: Vec<String>,
}

impl OrphanEntryPatternListVO {
    pub fn new(values: Vec<String>) -> Self {
        Self { values }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

// ─── AST parse results ─────────────────────────────────────────────────
//
// The AST value objects moved to `common::taxonomy_ast_vo` — their consumers
// (the Rust/Python/TS parsers) all live in `common`. Re-exported here so every
// existing `shared::orphan_rules::taxonomy_orphan_rules_vo::<AstType>` path
// keeps resolving without touching any call site.
pub use shared_common::taxonomy_ast_vo::*;

/// A set of file paths used for orphan-analysis reachability checks.
pub type FilePathSet = HashSet<FilePath>;

// ─── Violation payloads ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum AesOrphanViolation {
    TaxonomyOrphan {
        stem: String,
        category: &'static str,
        reason: Option<LintMessage>,
    },
    ContractOrphan {
        suffix: String,
        trait_name: String,
        target_layer: &'static str,
        reason: Option<LintMessage>,
    },
    CapabilitiesOrphan {
        stem: String,
        reason: Option<LintMessage>,
    },
    UtilityOrphan {
        stem: String,
        reason: Option<LintMessage>,
    },
    UtilityDeadCode {
        stem: String,
        imported_by: Vec<String>,
        reason: Option<LintMessage>,
    },
    AgentOrphan {
        agg_name: String,
        reason: Option<LintMessage>,
    },
    SurfaceOrphan {
        category: &'static str,
        stem: String,
        reason: Option<LintMessage>,
    },
}
