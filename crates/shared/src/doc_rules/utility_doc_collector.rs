// PURPOSE: Document collector — walk the document chain under an audit root and
// order the findings the capabilities report.
//
// The walk is filesystem work with no state and no rules, so it lives here as
// free functions: every rule answers over the same document set, and an agent
// may not perform I/O itself. `sorted` sits beside the collector for the same
// reason — it is a pure reduction over findings, not a decision about what a
// finding means.
use crate::taxonomy_doc_audit_context_vo::{MemberDescriptor, MemberRole};
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
/// BACKLOG pair, and the layer's DATA and BACKLOG pair. A layout that is
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

/// Collect the DATA/BACKLOG pair from the layer folder under *base*. A layout
/// with no shared layer carries no DATA document, which is not a finding.
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

/// Collect every classified member folder under *root*.
///
/// Classification keys on objective crate structure — never on the document
/// under audit — so a new surface crate that forgets `DESIGN.md` still owes
/// it.
pub fn collect_members(root: &Path) -> Vec<MemberDescriptor> {
    let mut out = Vec::new();
    let root_role = classify_member(root);
    if root_role != MemberRole::None {
        out.push(MemberDescriptor {
            path: root.to_path_buf(),
            role: root_role,
            present_docs: present_docs(root),
        });
    }
    for layout in FEATURE_LAYOUTS {
        let base = root.join(layout);
        let Ok(entries) = fs::read_dir(base) else {
            continue;
        };
        for entry in entries.flatten() {
            let folder = entry.path();
            if !folder.is_dir() {
                continue;
            }
            let role = classify_member(&folder);
            if role == MemberRole::None {
                continue;
            }
            let present_docs = present_docs(&folder);
            out.push(MemberDescriptor {
                path: folder,
                role,
                present_docs,
            });
        }
    }
    out
}

/// The top-level module folders owned by at least one PR-ID's project
/// identifier, across `crates|modules|packages`.
///
/// A folder counts as "owned" when its name matches the project identifier
/// segment of some declared PR-ID (`PR-<PROJECT>-NNN` → `PROJECT`). Only
/// top-level subfolders under each layout root are considered; `shared` and
/// `layer` are skipped as layer dirs, not project-owned feature folders.
/// Returns `None` when no layout root exists at all, leaving the caller
/// silent rather than flagging every declared PR-ID.
pub fn collect_pr_owned_module_folders(root: &Path) -> Option<BTreeSet<String>> {
    let mut owned: BTreeSet<String> = BTreeSet::new();
    let mut any_layout = false;
    for layout in FEATURE_LAYOUTS {
        let base = root.join(layout);
        let Ok(entries) = fs::read_dir(base) else {
            continue;
        };
        any_layout = true;
        for entry in entries.flatten() {
            let folder = entry.path();
            if !folder.is_dir() {
                continue;
            }
            let name = folder.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "shared" || name == "kernel" {
                continue;
            }
            owned.insert(name.to_string());
        }
    }
    if !any_layout {
        return None;
    }
    Some(owned)
}

/// Classify one folder by objective crate structure.
///
/// Layer: directory named `shared` or `kernel`.
/// Feature: contains `*agent_*_orchestrator*.rs` or exposes capability/agent
///   in Cargo.toml.
/// Surface: contains `surface_*` files or surface crate dependencies, not
///   layer/feature.
/// None: anything else.
fn classify_member(folder: &Path) -> MemberRole {
    let name = folder.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if name == "shared" || name == "kernel" {
        return MemberRole::Kernel;
    }
    if has_orchestrator_file(folder) || exposes_capability_in_cargo_toml(folder) {
        return MemberRole::Feature;
    }
    if has_surface_files(folder) || has_surface_cargo_deps(folder) {
        return MemberRole::Surface;
    }
    MemberRole::None
}

/// Does the folder's source tree contain an orchestrator file matching
/// `*agent_*_orchestrator*.rs`?
fn has_orchestrator_file(folder: &Path) -> bool {
    let Ok(entries) = fs::read_dir(folder) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.contains("agent_")
                    && name.contains("_orchestrator")
                    && name.ends_with(".rs")
                {
                    return true;
                }
            }
        }
        if path.is_dir() {
            if let Some(src) = path.file_name().and_then(|n| n.to_str()) {
                if src == "src" {
                    if let Ok(inner) = fs::read_dir(&path) {
                        for entry in inner.flatten() {
                            let p = entry.path();
                            if p.is_file() {
                                if let Some(n) = p.file_name().and_then(|n| n.to_str()) {
                                    if n.contains("agent_")
                                        && n.contains("_orchestrator")
                                        && n.ends_with(".rs")
                                    {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Does the folder's Cargo.toml expose a capability/agent module?
pub fn exposes_capability_in_cargo_toml(folder: &Path) -> bool {
    let cargo_path = folder.join("Cargo.toml");
    let Some(cargo) = cargo_path.to_str() else {
        return false;
    };
    let Ok(text) = fs::read_to_string(cargo) else {
        return false;
    };
    text.contains("agent_") || text.contains("capability") || text.contains("orchestrator")
}

/// Does the folder contain `surface_*` files?
fn has_surface_files(folder: &Path) -> bool {
    let Ok(entries) = fs::read_dir(folder) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("surface_") {
                    return true;
                }
            }
        }
        if path.is_dir() {
            if let Some(src) = path.file_name().and_then(|n| n.to_str()) {
                if src == "src" {
                    if let Ok(inner) = fs::read_dir(&path) {
                        for entry in inner.flatten() {
                            if let Some(n) = entry.path().file_name().and_then(|n| n.to_str()) {
                                if n.starts_with("surface_") {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Does the folder's Cargo.toml depend on surface crates?
pub fn has_surface_cargo_deps(folder: &Path) -> bool {
    let cargo_path = folder.join("Cargo.toml");
    let Some(cargo) = cargo_path.to_str() else {
        return false;
    };
    let Ok(text) = fs::read_to_string(cargo) else {
        return false;
    };
    text.contains("cli-commands")
        || text.contains("mcp-server")
        || text.contains("tui")
        || text.contains("dispatcher")
}

/// Document names actually present on disk in one folder.
fn present_docs(folder: &Path) -> BTreeSet<String> {
    let mut docs = BTreeSet::new();
    for name in [
        consts::FRD_DOC,
        consts::BACKLOG_DOC,
        consts::DATA_DOC,
        consts::DESIGN_DOC,
        consts::PRD_DOC,
        consts::ROADMAP_DOC,
        consts::README_DOC,
        consts::AGENTS_DOC,
        consts::ARCHITECTURE_DOC,
        consts::CONTRIBUTING_DOC,
    ] {
        if folder.join(name).is_file() {
            docs.insert(name.to_string());
        }
    }
    docs
}
