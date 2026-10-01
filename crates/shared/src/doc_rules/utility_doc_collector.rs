// PURPOSE: Document collector — walk the document chain under an audit root and
// order the findings the capabilities report.
//
// The walk is filesystem work with no state and no rules, so it lives here as
// free functions: every rule answers over the same document set, and an agent
// may not perform I/O itself. `sorted` sits beside the collector for the same
// reason — it is a pure reduction over findings, not a decision about what a
// finding means.
use crate::taxonomy_doc_rules_constant as consts;
use crate::taxonomy_doc_rules_request::{DocFinding, DocSource};

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

/// The root-level documents the document chain recognises.
const ROOT_DOCS: &[&str] = &[
    consts::PRD_DOC,
    consts::ROADMAP_DOC,
    consts::FRD_DOC,
    consts::DATA_DOC,
    consts::BACKLOG_DOC,
    consts::README_DOC,
    consts::AGENTS_DOC,
    consts::ARCHITECTURE_DOC,
    consts::CONTRIBUTING_DOC,
];

/// The feature layouts a workspace may organise its folders under.
const FEATURE_LAYOUTS: &[&str] = &["crates", "modules", "packages"];

/// Collect every document the chain recognises under *root*.
///
/// Three passes per layout: the root documents, the feature folder's FRD and
/// BACKLOG pair, and the kernel's DATA and BACKLOG pair. A layout that is
/// absent simply contributes nothing, which is what makes a sub-directory audit
/// self-contained rather than an error.
pub fn collect_documents(root: &Path) -> Vec<DocSource> {
    let mut out = Vec::new();
    for name in ROOT_DOCS {
        if let Some(source) = read_source(&root.join(name)) {
            out.push(source);
        }
    }
    for layout in FEATURE_LAYOUTS {
        let base = root.join(layout);
        collect_feature_docs(&base, &mut out);
        collect_shared_docs(&base, &mut out);
        collect_surface_docs(&base, &mut out);
    }
    out
}

/// The root master text, or `None` when the chain recognised no master.
///
/// The master is the first recognised document whose name is a master
/// candidate, so a workspace that still carries a root `BACKLOG.md` keeps
/// working while it migrates to `ROADMAP.md`.
pub fn master_text(documents: &[DocSource]) -> Option<String> {
    documents
        .iter()
        .find(|document| {
            document
                .path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| consts::MASTER_DOC_CANDIDATES.contains(&name))
        })
        .map(|document| document.text.clone())
}

/// Deduplicate and sort findings so two runs over the same tree report the
/// same violations in the same order.
pub fn sorted(findings: Vec<DocFinding>) -> Vec<DocFinding> {
    let mut seen = BTreeSet::new();
    let mut out: Vec<DocFinding> = findings
        .into_iter()
        .filter(|finding| {
            seen.insert((
                finding.code,
                finding.violation_type,
                finding.doc.clone(),
                finding.message.clone(),
            ))
        })
        .collect();
    out.sort_by(|a, b| {
        (&a.doc, a.code, a.violation_type, a.message.clone()).cmp(&(
            &b.doc,
            b.code,
            b.violation_type,
            b.message.clone(),
        ))
    });
    out
}

/// Read one document, returning `None` when it is absent or unreadable. A
/// document that cannot be read is a parse skip, never a violation.
fn read_source(path: &Path) -> Option<DocSource> {
    let text = fs::read_to_string(path).ok()?;
    Some(DocSource {
        path: path.to_path_buf(),
        text,
    })
}

/// Collect the FRD/BACKLOG pair from every feature folder under *base*.
fn collect_feature_docs(base: &Path, out: &mut Vec<DocSource>) {
    let Ok(entries) = fs::read_dir(base) else {
        return;
    };
    for entry in entries.flatten() {
        let feature = entry.path();
        if !feature.is_dir() {
            continue;
        }
        for doc in [consts::FRD_DOC, consts::BACKLOG_DOC] {
            if let Some(source) = read_source(&feature.join(doc)) {
                out.push(source);
            }
        }
    }
}

/// Collect the DATA/BACKLOG pair from the kernel folder under *base*. A layout
/// with no shared kernel carries no DATA document, which is not a finding.
fn collect_shared_docs(base: &Path, out: &mut Vec<DocSource>) {
    let kernel = base.join(consts::KERNEL_DIR);
    if !kernel.is_dir() {
        return;
    }
    for doc in [consts::DATA_DOC, consts::BACKLOG_DOC] {
        if let Some(source) = read_source(&kernel.join(doc)) {
            out.push(source);
        }
    }
}

/// Collect DESIGN.md and its paired BACKLOG.md from every surface folder under
/// *base*. A folder is a surface folder only when it holds a DESIGN document.
fn collect_surface_docs(base: &Path, out: &mut Vec<DocSource>) {
    let Ok(entries) = fs::read_dir(base) else {
        return;
    };
    for entry in entries.flatten() {
        let folder = entry.path();
        if !folder.is_dir() || !folder.join(consts::DESIGN_DOC).is_file() {
            continue;
        }
        if let Some(source) = read_source(&folder.join(consts::DESIGN_DOC)) {
            out.push(source);
        }
        if let Some(source) = read_source(&folder.join(consts::BACKLOG_DOC)) {
            out.push(source);
        }
    }
}
