// PURPOSE: DocAuditContext — the shared per-audit state every doc capability reads.
//
// One audit run walks the document chain once and hands the result to five
// capabilities, each answering one AES doc rule. The context is the single
// object they all receive, so the per-document derivations that would
// otherwise be repeated five times — the file name, the path relative to the
// audit root, and the document stamp on every finding — are computed once here.
//
// The context carries no rules. Which documents a capability inspects, and
// what it reports when one drifts, is that capability's decision.
use crate::taxonomy_doc_rules_request::{DocFinding, DocSource};

use std::path::{Path, PathBuf};

/// One audit run's shared state: the recognised documents, the root they were
/// collected under, and the root master text the cross-cutting rules resolve
/// against.
#[derive(Clone, Debug)]
pub struct DocAuditContext {
    /// The audit root every relative path is rendered against.
    root: PathBuf,
    /// Every document the document chain recognised under `root`.
    documents: Vec<DocSource>,
    /// The root master text, when the chain recognised a master document.
    master: Option<String>,
}

impl DocAuditContext {
    /// Build the context for one audit run.
    pub fn new(root: &Path, documents: Vec<DocSource>, master: Option<String>) -> Self {
        Self {
            root: root.to_path_buf(),
            documents,
            master,
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

    /// Stamp every finding in *findings* with the document it came from, so a
    /// consumer can group the report by file without re-deriving the path.
    pub fn stamp(&self, document: &DocSource, findings: &mut [DocFinding]) {
        let relative = self.relative_path(document);
        for finding in findings.iter_mut() {
            finding.doc = relative.clone();
        }
    }
}
