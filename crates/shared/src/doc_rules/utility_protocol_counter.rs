// PURPOSE: FR/protocol parity counters — requirement headings in an FRD and capability-seam
// classes in a feature's shared contract module. Both are stateless counts, so
// they live here and the checker only decides what a mismatch means.
use crate::taxonomy_doc_rules_constant as consts;
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

/// Number of `FR-<Feature>-NNN:` requirement headings in the document.
///
/// The same heading shape the FR-ID rule accepts, so a requirement nested
/// under a subsection is still counted once.
pub fn count_fr_headings(text: &str) -> Option<usize> {
    let re = fr_id_heading_re()?;
    Some(re.find_iter(text).count())
}

/// True when the text contains at least one requirement heading in the
/// shared FR-ID shape; false otherwise.
pub fn has_fr_headings(text: &str) -> bool {
    fr_id_heading_re().is_some_and(|re| re.is_match(text))
}

/// The kernel `src` roots the shared contract modules live in: the feature's
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

/// Number of `pub trait I*Protocol` declarations in the feature's shared
/// module, walking the module recursively.
///
/// A protocol file may declare many classes, so every `.rs` file in the
/// module tree is scanned; a class organized in a subdirectory is counted
/// like one at the top level. `I*Aggregate` traits are composite entry
/// points rather than capability seams and are excluded. A module that
/// cannot be read returns `None`, which leaves the caller with nothing to
/// compare against.
pub fn count_protocol_traits(module_dir: &Path) -> Option<usize> {
    let mut count = 0;
    walk_rs_files(module_dir, &mut |_path, text| {
        count += text
            .lines()
            .filter(|line| {
                let line = line.trim();
                line.starts_with("pub trait I")
                    && line.contains("Protocol")
                    && !line.contains("Aggregate")
            })
            .count();
    })?;
    Some(count)
}

/// Recursively walk `dir` applying *visit* to every readable `.rs` file.
///
/// `None` is returned only when the starting directory itself cannot be
/// read; unreadable entries deeper in the walk are skipped, matching the
/// parse-skip convention.
fn walk_rs_files(dir: &Path, visit: &mut dyn FnMut(&Path, &str)) -> Option<()> {
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if walk_rs_files(&path, visit).is_none() {
                continue;
            }
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            visit(&path, &text);
        }
    }
    Some(())
}
