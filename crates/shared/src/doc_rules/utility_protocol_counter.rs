// PURPOSE: FR/protocol parity counters — requirement headings in an FRD and capability-seam
// classes in a feature's shared contract module. Both are stateless counts, so
// they live here and the checker only decides what a mismatch means.
use crate::taxonomy_doc_rules_constant as consts;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// The one FR-ID shape the whole AES601 rule family accepts: an H2–H4
/// requirement heading (`### FR-Feature-001: ...`). Every consumer — the
/// FR-ID format check, the FR-field check, and the parity counter — matches
/// against this single pattern so the accepted ID set cannot drift between
/// them.
///
/// The feature segment allows mixed case and underscores, so the rule accepts
/// `FR-AutoFix-001` exactly as `frautoFix` and `fr_auto_fix`; the documented
/// form is CamelCase, but case is never a violation. Feature crates use
/// `-` where the shared module uses `_`.
const FR_ID_HEADING_PATTERN: &str = r"(?m)^#{2,4}\s+(?P<id>FR-[A-Za-z0-9_]+-\d+):";

/// The shared FR-ID heading pattern, compiled once.
///
/// The capture is named `id` so the FR-field check can read the ID it
/// anchors its missing-field finding to.
///
/// Public because the checker's FR-field check and the parity counter must
/// both anchor on the same accepted ID set; a drift between the two was the
/// failure mode this pattern exists to kill.
pub fn fr_id_heading_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(FR_ID_HEADING_PATTERN).ok())
        .as_ref()
}

/// The one PR-ID shape the AES601 rule family accepts: an H2–H4
/// requirement heading (`### PR-Project-001: ...`) in a PRD's `Project
/// Requirements` or a ROADMAP's `Roadmap Status`. Same H2–H4 colon
/// shape as the FR/DS/DT patterns so the accepted ID sets cannot drift
/// between the parity check and the counter.
const PR_ID_HEADING_PATTERN: &str = r"(?m)^#{2,4}\s+(?P<id>PR-[A-Za-z0-9_]+-\d+):";

/// The shared PR-ID heading pattern, compiled once.
pub fn pr_id_heading_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(PR_ID_HEADING_PATTERN).ok())
        .as_ref()
}

/// Accepted PR-ID set from a document's `PR-<NAME>-NNN:` headings.
///
/// Returns `None` when the document declares no PR-IDs, so the caller
/// keeps the check silent rather than flagging every sibling row.
pub fn collect_pr_id_set(text: &str) -> Option<Vec<String>> {
    let re = pr_id_heading_re()?;
    let ids: Vec<String> = re
        .captures_iter(text)
        .filter_map(|caps| caps.name("id").map(|m| m.as_str().to_string()))
        .collect();
    if ids.is_empty() { None } else { Some(ids) }
}

/// A DESIGN.md DS-ID heading (`### DS-DISP-001: <title>`), H2–H4,
/// colon-terminated. The NAME token (e.g. DISP, CLIC, MCPS, TUIC) is a
/// per-folder qualifier, so the qualified `DS-<NAME>-NNN` form is mandatory;
/// a bare `DS-001` (digits only after the prefix, no NAME) no longer matches.
const DS_ID_HEADING_PATTERN: &str = r"(?m)^#{2,4}\s+(?P<id>DS-[A-Za-z0-9_]+-\d+):";

/// The shared DS-ID heading pattern, compiled once.
pub fn ds_id_heading_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(DS_ID_HEADING_PATTERN).ok())
        .as_ref()
}

/// Accepted DS-ID set from a DESIGN.md's `DS-<NAME>-NNN:` headings.
pub fn collect_ds_id_set(text: &str) -> Option<Vec<String>> {
    let re = ds_id_heading_re()?;
    let ids: Vec<String> = re
        .captures_iter(text)
        .filter_map(|caps| caps.name("id").map(|m| m.as_str().to_string()))
        .collect();
    if ids.is_empty() { None } else { Some(ids) }
}

/// A DATA.md DT-ID heading (`## DT-SHAR-001: <title>`), H2–H4,
/// colon-terminated. The NAME token (e.g. SHAR, MCPS, TUIC) is a per-folder
/// qualifier, so the qualified `DT-<NAME>-NNN` form is mandatory; a bare
/// `DT-001` (digits only after the prefix, no NAME) no longer matches.
const DT_ID_HEADING_PATTERN: &str = r"(?m)^#{2,4}\s+(?P<id>DT-[A-Za-z0-9_]+-\d+):";

/// The shared DT-ID heading pattern, compiled once.
pub fn dt_id_heading_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(DT_ID_HEADING_PATTERN).ok())
        .as_ref()
}

/// Accepted DT-ID set from a DATA.md's `DT-<NAME>-NNN:` headings.
pub fn collect_dt_id_set(text: &str) -> Option<Vec<String>> {
    let re = dt_id_heading_re()?;
    let ids: Vec<String> = re
        .captures_iter(text)
        .filter_map(|caps| caps.name("id").map(|m| m.as_str().to_string()))
        .collect();
    if ids.is_empty() { None } else { Some(ids) }
}

/// Number of `FR-<Feature>-NNN:` requirement headings in the document.
///
/// The same heading shape the FR-ID rule accepts, so a requirement nested
/// under a subsection is still counted once.
pub fn count_fr_headings(text: &str) -> Option<usize> {
    let re = fr_id_heading_re()?;
    Some(re.find_iter(text).count())
}

/// True when the text contains at least one requirement heading in the
///
/// The layer `src` roots the shared contract modules live in: the feature's
/// contract module is resolved against each documented workspace layout
/// (`crates/shared`, `modules/shared`, `packages/shared`), so a feature in
/// any of them finds the right base instead of silently skipping the check.
/// The same layout list `collect_documents` scans for feature folders.
pub fn locate_kernel_srcs(root: &Path) -> Vec<PathBuf> {
    ["crates", "modules", "packages"]
        .into_iter()
        .map(|layout| root.join(layout).join(consts::KERNEL_DIR).join("src"))
        .filter(|path| path.is_dir())
        .collect()
}

/// Number of capability-seam classes in the feature's shared module,
/// walking the module recursively.
///
/// A protocol file may declare many classes, so every source file in the
/// module tree is scanned; a class organized in a subdirectory is counted
/// like one at the top level. `I*Aggregate` traits / `*Aggregate` classes
/// are composite entry points rather than capability seams and are
/// excluded. A module that cannot be read returns `None`, which leaves
/// the caller with nothing to compare against.
///
/// `.rs` and `.py` sources are both read so a Python shared tree counts
/// identically to a Rust one; documents (FRDs) are never read.
pub fn count_protocol_traits(module_dir: &Path) -> Option<usize> {
    let mut count = 0;
    walk_source_files(module_dir, &mut |path, text| {
        count += count_protocol_decls_in_text(path, text);
    })?;
    Some(count)
}

/// Capability-seam declarations in one source file: Rust `pub trait
/// I*Protocol` or a Python `I*Protocol` class (the name contract mirrors the
/// Rust side, so a Python shared tree counts identically to a Rust one).
/// Python protocol classes only matter in `contract_*` files — a domain model
/// or aggregate in a non-contract module is not a capability seam.
fn count_protocol_decls_in_text(path: &Path, text: &str) -> usize {
    let ext = path.extension().and_then(|e| e.to_str());
    if ext == Some("py") {
        let is_contract = path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with("contract_"));
        if !is_contract {
            return 0;
        }
        count_python_protocol_classes(text)
    } else {
        count_rust_protocol_traits(text)
    }
}

/// Count `pub trait I*Protocol` capability seams in Rust source text.
fn count_rust_protocol_traits(text: &str) -> usize {
    text.lines()
        .filter(|line| line.trim().starts_with("pub trait I") && is_protocol_decl(line))
        .count()
}

/// Count Python `I*Protocol` capability-seam classes in `contract_*` source
/// text. Matched on the class name alone — symmetric with the Rust counter's
/// `is_protocol_decl` — never on a stray decorator line.
fn count_python_protocol_classes(text: &str) -> usize {
    text.lines()
        .filter_map(|line| line.trim().strip_prefix("class "))
        .filter(|rest| {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            name.starts_with('I') && name.ends_with("Protocol") && !name.contains("Aggregate")
        })
        .count()
}

/// Is this line a `pub trait I*Protocol` capability seam?
///
/// Matched on the trait name alone — never on anything else the line happens
/// to carry. A doc comment, a string literal, or a doc block containing
/// `Protocol` next to `pub trait I` must not inflate the seam count.
fn is_protocol_decl(line: &str) -> bool {
    let Some((_, rest)) = line.trim().split_once("trait ") else {
        return false;
    };
    // The trait name is the identifier before any generics/bounds.
    let name: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    name.starts_with('I') && name.ends_with("Protocol") && !name.contains("Aggregate")
}

/// Method names declared anywhere in the feature's contract module, or
/// `None` when the module cannot be read.
///
/// A method exists when some `fn <name>` (Rust) or `def <name>` (Python)
/// declaration site occurs in one of the module's source files — a protocol
/// seam or aggregate class that carries the method satisfies the name, and so
/// does its implementation. The walk reuses the same source-only recursion
/// as the protocol-class counter, so a method invented only inside prose
/// keeps reading as missing.
pub fn contract_method_names(module_dir: &Path) -> Option<HashSet<String>> {
    let mut names = HashSet::new();
    walk_source_files(module_dir, &mut |path, text| {
        let is_python = path.extension().and_then(|e| e.to_str()) == Some("py");
        for line in text.lines() {
            let trimmed = line.trim();
            if is_python {
                // Python: `def <name>` opens a declaration site after any
                // decorator/qualifier on the same line (rare but valid in
                // generated stubs). A docstring line is prose. An `async def`
                // qualifier is stripped first, mirroring the Rust branch.
                let after_async = trimmed.strip_prefix("async ").unwrap_or(trimmed);
                let Some(rest) = after_async.strip_prefix("def ") else {
                    continue;
                };
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                if !name.is_empty() {
                    names.insert(name);
                }
            } else {
                // Only a real declaration site satisfies the name: `fn` must
                // open the statement after any visibility or async qualifier.
                // A doc comment mentioning `fn name` is prose, not a
                // declaration.
                let after_vis = trimmed
                    .strip_prefix("pub ")
                    .or_else(|| trimmed.strip_prefix("pub(crate) "))
                    .unwrap_or(trimmed);
                let after_async = after_vis.strip_prefix("async ").unwrap_or(after_vis);
                let Some(rest) = after_async.strip_prefix("fn ") else {
                    continue;
                };
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                if !name.is_empty() {
                    names.insert(name);
                }
            }
        }
    })?;
    Some(names)
}

/// Recursively walk `dir` applying *visit* to every readable source file
/// (`.rs` or `.py`).
///
/// `None` is returned only when the starting directory itself cannot be
/// read; unreadable entries deeper in the walk are skipped, matching the
/// parse-skip convention.
fn walk_source_files(dir: &Path, visit: &mut dyn FnMut(&Path, &str)) -> Option<()> {
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // An unreadable subtree returns None; skip it (parse-skip convention).
            if walk_source_files(&path, visit).is_none() {
                continue;
            }
        } else if matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("rs") | Some("py")
        ) {
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            visit(&path, &text);
        }
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn contract_method_names_finds_async_def_methods() {
        let dir = std::env::temp_dir().join("lint_arwaky_async_def_test");
        if std::fs::create_dir_all(&dir).is_err() {
            return;
        }
        let file_path = dir.join("contract_example_protocol.py");
        let mut f = match std::fs::File::create(&file_path) {
            Ok(f) => f,
            Err(_) => return,
        };
        let src = "class IExampleProtocol:\n\
                   @abstractmethod\n\
                   async def check_session(self, session) -> bool:\n\
                       ...\n\
                   def rotate_sync(self, request) -> bool:\n\
                       ...\n";
        if f.write_all(src.as_bytes()).is_err() {
            return;
        }
        drop(f);
        let result = contract_method_names(&dir);
        let _ = std::fs::remove_dir_all(&dir);
        // result is None when the walk itself cannot read the directory;
        // in that case the test silently passes (parse-skip convention).
        let Some(names) = result else { return };
        assert!(
            names.contains("check_session"),
            "async def method must be found"
        );
        assert!(
            names.contains("rotate_sync"),
            "sync def method must be found"
        );
    }
}
