// PURPOSE: ScanRequest VO — request payload for the analysis pipeline
use crate::taxonomy_format_vo::Format;

/// Target path for the scan.
pub struct ScanTarget {
    pub value: String,
}

impl ScanTarget {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Default for ScanTarget {
    fn default() -> Self {
        Self {
            value: ".".to_string(),
        }
    }
}

/// Mode of analysis to run.
#[derive(Debug, Clone, Default)]
pub enum ScanMode {
    #[default]
    Check,
    Scan,
    Ci {
        threshold: u32,
    },
}

/// What a user-supplied path resolves to, and therefore how much the scan
/// must cover versus how much of the result is reported.
///
/// The distinction matters because the architecture rules need the whole
/// member dir: a subfolder scan that narrowed the *scan* to the subfolder
/// would miss a missing import declared in a sibling file. So a subfolder or
/// single-file target widens the scan to its top-level member dir and filters
/// only the reported violations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanScope {
    /// Workspace root: scan every member, report everything.
    Workspace,
    /// `crates` / `packages` / `modules`: scan that member dir, report all.
    TopLevel { member: String },
    /// A subfolder inside a member dir: scan the whole member dir, report only
    /// violations under the subfolder.
    Subfolder { scan_root: String, filter_to: String },
    /// One file: scan the whole member dir, report only that file.
    SingleFile { scan_root: String, filter_to: String },
}

impl ScanScope {
    /// The directory the linters must walk.
    pub fn scan_root(&self) -> &str {
        match self {
            Self::Workspace | Self::TopLevel { .. } => "",
            Self::Subfolder { scan_root, .. } | Self::SingleFile { scan_root, .. } => scan_root,
        }
    }

    /// Narrow reported violations to this prefix; `None` reports everything.
    pub fn filter_prefix(&self) -> Option<&str> {
        match self {
            Self::Workspace | Self::TopLevel { .. } => None,
            Self::Subfolder { filter_to, .. } | Self::SingleFile { filter_to, .. } => {
                Some(filter_to)
            }
        }
    }
}

/// The three member directories a workspace is built from.
pub const MEMBER_DIRS: [&str; 3] = ["crates", "packages", "modules"];

/// Classify a target path relative to the workspace root it sits in.
///
/// `root` is the workspace root (or the member dir when the user targeted one),
/// `target` is the user-supplied path. Returns the scope plus, for the
/// subfolder and single-file cases, the member dir to scan.
pub fn classify_scan_scope(root: &str, target: &str) -> ScanScope {
    let root_path = std::path::Path::new(root);
    let target_path = std::path::Path::new(target);

    // A file target: scan its member dir, report only the file.
    if target_path.is_file() {
        return match member_dir_of(root_path, target_path) {
            Some(scan_root) => ScanScope::SingleFile {
                scan_root,
                filter_to: target.to_string(),
            },
            None => ScanScope::Workspace,
        };
    }

    // The target itself is a member dir: scan and report it whole.
    if let Some(name) = target_path.file_name().and_then(|n| n.to_str())
        && MEMBER_DIRS.contains(&name)
    {
        return ScanScope::TopLevel {
            member: name.to_string(),
        };
    }

    // A subfolder inside a member dir: scan the member dir, report the subfolder.
    if let Some(scan_root) = member_dir_of(root_path, target_path) {
        return ScanScope::Subfolder {
            scan_root,
            filter_to: target.to_string(),
        };
    }

    ScanScope::Workspace
}

/// The member dir (`crates` / `packages` / `modules`) that contains `target`,
/// when one does. A target that *is* a member dir returns `None` — the caller
/// handles that case as `TopLevel`.
fn member_dir_of(root: &std::path::Path, target: &std::path::Path) -> Option<String> {
    let relative = target.strip_prefix(root).ok()?;
    let mut ancestor = relative.components();
    let first = ancestor.next()?.as_os_str().to_str()?;
    MEMBER_DIRS
        .contains(&first)
        .then(|| root.join(first).to_string_lossy().into_owned())
}

/// Request to run the full analysis pipeline.
pub struct ScanRequest {
    pub target: ScanTarget,
    pub mode: ScanMode,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub format: Format,
}

impl ScanRequest {
    pub fn new(target: ScanTarget, mode: ScanMode) -> Self {
        Self {
            target,
            mode,
            filter: None,
            member: None,
            format: Format::Text,
        }
    }
}
