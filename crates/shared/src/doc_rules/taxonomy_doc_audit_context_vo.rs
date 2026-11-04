// PURPOSE: DocAuditContext — the shared per-audit state every doc capability reads.
//
// One audit run walks the document chain once and hands the result to nine
// capabilities, each answering one AES doc rule. The context is the single
// object they all receive, so the per-document derivations that would
// otherwise be repeated nine times — the file name, the path relative to the
// audit root, and the document stamp on every finding — are computed once here.
//
// The context carries no rules. Which documents a capability inspects, and
// what it reports when one drifts, is that capability's decision.
use crate::taxonomy_doc_rules_request::{DocFinding, DocSource};

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// One workspace member folder, classified and pre-populated by the collector.
///
/// Classification keys on objective crate structure (orchestrator files,
/// `surface_*` files, directory name) — never on the document under audit —
/// so a new surface crate that forgets `DESIGN.md` still owes it.
#[derive(Clone, Debug)]
pub struct MemberDescriptor {
    /// Path relative to the audit root, e.g. `crates/billing`.
    pub path: PathBuf,
    /// The architectural role, derived by objective crate structure.
    pub role: MemberRole,
    /// Document names actually present on disk in this member (e.g. `{"FRD.md"}`).
    pub present_docs: BTreeSet<String>,
}

/// Architectural role of a workspace member folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MemberRole {
    /// Owns a vertical slice: `FRD.md` + `BACKLOG.md`.
    Feature,
    /// Shared layer: `DATA.md` + `BACKLOG.md`.
    Kernel,
    /// Surface/utility crate: `DESIGN.md` + `BACKLOG.md`.
    Surface,
    /// Not a documentation-owing member (e.g. a build-only workspace crate).
    None,
}

/// Doc-rules configuration loaded from `lint_arwaky.config.yaml`.
///
/// Defaults are staged `false` in Stages 1–2 (migration window) and flipped
/// to `true` in Stage 3 (Decision Q4).
#[derive(Clone, Debug)]
pub struct DocRulesConfig {
    /// When true, member doc presence (FR-DOC-008) is strictly enforced.
    pub strict_member_docs: bool,
    /// When true, placeholder tokens (`<...>`, `{...}`) are reported as violations.
    pub strict_placeholders: bool,
    /// Member paths exempted from FR-DOC-008 checks.
    pub exempt_members: Vec<String>,
}

impl DocRulesConfig {
    /// Staged config used until Stage 3 flips strictness on.
    pub fn staged() -> Self {
        Self {
            strict_member_docs: false,
            strict_placeholders: false,
            exempt_members: Vec::new(),
        }
    }
}

/// How the CLI `--strict` flag and config-file defaults combine for one run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrictModeOverride {
    /// Apply whatever `strict_member_docs` / `strict_placeholders` the yaml says.
    UseConfigFile,
    /// `--strict` flag: treat both strict toggles as `true` for this invocation.
    ForceStrict,
}

/// One audit run's shared state: the recognised documents, the root they were
/// collected under, the root master text, and the pre-collected member
/// descriptors plus strictness configuration.
#[derive(Clone, Debug)]
pub struct DocAuditContext {
    /// The audit root every relative path is rendered against.
    root: PathBuf,
    /// Every document the document chain recognised under `root`.
    documents: Vec<DocSource>,
    /// The root master text, when the chain recognised a master document.
    master: Option<String>,
    /// Pre-collected member descriptors; the capabilities perform zero I/O.
    members: Vec<MemberDescriptor>,
    /// Config-file strictness toggles and exemptions, or `None` when unconfigured.
    config: Option<DocRulesConfig>,
    /// The CLI-issued strictness override for this run.
    strict_override: StrictModeOverride,
}

impl DocAuditContext {
    /// Build the context for one audit run.
    pub fn new(
        root: &Path,
        documents: Vec<DocSource>,
        master: Option<String>,
        members: Vec<MemberDescriptor>,
        config: Option<DocRulesConfig>,
        strict_override: StrictModeOverride,
    ) -> Self {
        Self {
            root: root.to_path_buf(),
            documents,
            master,
            members,
            config,
            strict_override,
        }
    }

    /// The audit root, used to render document paths and to resolve a
    /// feature's shared contract module.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Every recognised document, in collection order.
    pub fn documents(&self) -> &[DocSource] {
        &self.documents
    }

    /// The root master text, or `None` when the workspace has no master
    /// document — a sub-directory audit is self-contained and resolves against
    /// its own root.
    pub fn master(&self) -> Option<&str> {
        self.master.as_deref()
    }

    /// The document's own file name (`FRD.md`, `BACKLOG.md`, …). The name
    /// borrows from *document*, whose path owns it.
    pub fn document_name<'a>(&self, document: &'a DocSource) -> &'a str {
        document
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
    }

    /// The document path relative to the audit root, with `\` normalised to
    /// `/` so a finding's `doc` reads the same on every platform.
    pub fn relative_path(&self, document: &DocSource) -> String {
        document
            .path
            .strip_prefix(&self.root)
            .unwrap_or(&document.path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    /// True when the document sits directly in the audit root rather than in a
    /// feature folder. The root is the one place the state vocabulary lives.
    pub fn is_root_document(&self, document: &DocSource) -> bool {
        document
            .path
            .parent()
            .is_some_and(|parent| parent == self.root)
    }

    /// Pre-collected member descriptors; capabilities read this field, never
    /// walk the filesystem themselves.
    pub fn members(&self) -> &[MemberDescriptor] {
        &self.members
    }

    /// Doc-rules configuration, or `None` when no config file is present.
    pub fn config(&self) -> Option<&DocRulesConfig> {
        self.config.as_ref()
    }

    /// The strictness override for this run.
    pub fn strict_override(&self) -> StrictModeOverride {
        self.strict_override
    }

    /// Effective strict member-doc enforcement: `true` when `--strict` was
    /// passed or the config file enables it.
    pub fn effective_strict_member_docs(&self) -> bool {
        match self.strict_override {
            StrictModeOverride::ForceStrict => true,
            StrictModeOverride::UseConfigFile => {
                self.config.as_ref().is_some_and(|c| c.strict_member_docs)
            }
        }
    }

    /// Effective strict placeholder enforcement: `true` when `--strict` was
    /// passed or the config file enables it.
    pub fn effective_strict_placeholders(&self) -> bool {
        match self.strict_override {
            StrictModeOverride::ForceStrict => true,
            StrictModeOverride::UseConfigFile => {
                self.config.as_ref().is_some_and(|c| c.strict_placeholders)
            }
        }
    }

    /// True when the audit root carries at least one workspace-root marker file
    /// (e.g. `Cargo.toml`, `package.json`, `pyproject.toml`).
    pub fn is_workspace_root(&self) -> bool {
        crate::taxonomy_doc_rules_constant::WORKSPACE_ROOT_MARKERS
            .iter()
            .any(|marker| self.root.join(marker).exists())
    }

    /// Stamp every finding in *findings* with the document it came from, so a
    /// consumer can group the report by file without re-deriving the path.
    pub fn stamp(&self, document: &DocSource, findings: &mut [DocFinding]) {
        let relative = self.relative_path(document);
        for finding in findings.iter_mut() {
            finding.doc = relative.clone();
        }
    }
}
