// PURPOSE: FR/protocol parity counters — requirement headings in an FRD and capability-seam
// classes in a feature's shared contract module. Both are stateless counts, so
// they live here and the checker only decides what a mismatch means.
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

/// The FR-heading matcher, compiled once for the whole process.
fn fr_heading_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"(?m)^#{2,4}\s+FR-[A-Za-z0-9_]+-\d+:").ok())
        .as_ref()
}

/// Number of `FR-<Feature>-NNN:` headings in the document.
///
/// The same heading shape the FR-ID rule accepts, so a requirement nested
/// under a subsection is still counted once. Feature names are CamelCase
/// (`FR-AutoFix-001`), so the feature segment allows mixed case.
pub fn count_fr_headings(text: &str) -> Option<usize> {
    let re = fr_heading_re()?;
    Some(re.find_iter(text).count())
}

/// Number of `pub trait I*Protocol` declarations in the feature's shared module.
///
/// A protocol file may declare many classes, so every `.rs` file in the module
/// is scanned. `I*Aggregate` traits are composite entry points rather than
/// capability seams and are excluded. A module that cannot be read returns
/// `None`, which leaves the caller with nothing to compare against.
pub fn count_protocol_traits(module_dir: &Path) -> Option<usize> {
    let entries = fs::read_dir(module_dir).ok()?;
    let mut count = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        count += count_protocol_traits_in_text(&text);
    }
    Some(count)
}

/// Count `pub trait I*Protocol` declarations in already-read source text.
fn count_protocol_traits_in_text(text: &str) -> usize {
    text.lines()
        .filter(|line| {
            let line = line.trim();
            line.starts_with("pub trait I")
                && line.contains("Protocol")
                && !line.contains("Aggregate")
        })
        .count()
}
