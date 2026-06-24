// PURPOSE: SurfacesOrphanAnalyzer — ISurfacesOrphanProtocol for orphan surface detection
// Surface orphan detection per category:
// - Smart surface (_command, _controller, _page, _entry): must be imported by entry or router
// - Utility surface (_hook, _store, _action, _screen, _router): must be imported by smart surface
// - Passive surface (_component, _view, _layout): must be imported by smart or utility surface
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::orphan_detector::contract_orphan_protocol::ISurfacesOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_definition_vo::LayerDefinition;

pub struct SurfacesOrphanAnalyzer {}

impl Default for SurfacesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfacesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl ISurfacesOrphanProtocol for SurfacesOrphanAnalyzer {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult {
        is_surface_orphan(f, alive_files, definition)
    }
}

/// Get surface suffix from filename
fn get_surface_suffix(basename: &str) -> String {
    let stem = basename
        .replace(".rs", "")
        .replace(".py", "")
        .replace(".ts", "")
        .replace(".js", "");
    let parts: Vec<&str> = stem.split('_').collect();
    if parts.len() >= 2 {
        parts[parts.len() - 1].to_string()
    } else {
        String::new()
    }
}

/// Surface category
fn surface_category(suffix: &str) -> &'static str {
    match suffix {
        "command" | "controller" | "page" | "entry" => "smart",
        "hook" | "store" | "action" | "screen" | "router" => "utility",
        "component" | "view" | "layout" => "passive",
        _ => "unknown",
    }
}

pub fn is_surface_orphan(
    f: &FilePath,
    alive_files: &ReachabilityResult,
    _definition: Option<&LayerDefinition>,
) -> OrphanIndicatorResult {
    let alive: Vec<String> = alive_files
        .paths
        .iter()
        .map(|fp| fp.value().to_string())
        .collect();
    let is_reachable = alive.contains(&f.value().to_string());
    if is_reachable {
        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
    }

    // Check if imported by entry or router
    let fp_val = f.value();
    let basename = match std::path::Path::new(fp_val)
        .file_name()
        .and_then(|n| n.to_str())
    {
        Some(n) => n,
        None => "",
    };
    let stem = basename
        .replace(".rs", "")
        .replace(".py", "")
        .replace(".ts", "")
        .replace(".js", "");

    if let Ok(content) = std::fs::read_to_string(fp_val) {
        // Check if this surface is imported by any entry or router file
        let root = std::path::Path::new(".");
        if let Ok(workspace_root) =
            crate::capabilities_orphan_capabilities_analyzer::find_workspace_root(root)
        {
            if let Ok(imported) = check_imported_by_entry_or_router(&workspace_root, &stem) {
                if imported {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }
            }

            // Also check for function/struct names from the surface file
            let mut identifiers: Vec<String> = Vec::new();
            // Extract pub fn names
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("pub fn ") {
                    if let Some(name) = trimmed.strip_prefix("pub fn ") {
                        if let Some(name) = name.split('(').next() {
                            identifiers.push(name.trim().to_string());
                        }
                    }
                }
                // Extract pub struct names
                if trimmed.starts_with("pub struct ") {
                    if let Some(name) = trimmed.strip_prefix("pub struct ") {
                        if let Some(name) = name.split('{').next() {
                            identifiers.push(name.trim().to_string());
                        }
                    }
                }
            }

            for id in &identifiers {
                if let Ok(imported) = check_imported_by_entry_or_router(&workspace_root, id) {
                    if imported {
                        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                    }
                }
            }
        }
    }

    let suffix = get_surface_suffix(basename);
    let category = surface_category(&suffix);
    OrphanIndicatorResult::new(
        true,
        AesOrphanViolation::SurfaceOrphan {
            category,
            stem,
            reason: Some("Surface is unreachable.".into()),
        }
        .to_string(),
        Severity::HIGH,
    )
}

fn check_imported_by_entry_or_router(
    workspace_root: &std::path::Path,
    stem: &str,
) -> Result<bool, std::io::Error> {
    for dir_name in &["crates", "packages", "modules"] {
        let dir = workspace_root.join(dir_name);
        if dir.is_dir() && check_dir_imports(&dir, stem)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn check_dir_imports(dir: &std::path::Path, stem: &str) -> Result<bool, std::io::Error> {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if check_dir_imports(&path, stem)? {
                    return Ok(true);
                }
            } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let is_entry_or_router = name.starts_with("root_")
                    || name.starts_with("cli_")
                    || name.starts_with("mcp_")
                    || name.contains("_entry")
                    || name.contains("_router");
                if is_entry_or_router
                    && (name.ends_with(".rs")
                        || name.ends_with(".py")
                        || name.ends_with(".ts")
                        || name.ends_with(".js"))
                {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if content.contains(stem) {
                            return Ok(true);
                        }
                    }
                }
            }
        }
    }
    Ok(false)
}

pub fn is_surface_orphan_raw(f: &FilePath, all_files: &[String]) -> OrphanIndicatorResult {
    let fp = f.value();
    let basename = match fp.split('/').next_back() {
        Some(b) => b,
        None => "",
    };
    let suffix = get_surface_suffix(basename);
    let category = surface_category(&suffix);
    let stem = basename
        .replace(".rs", "")
        .replace(".py", "")
        .replace(".ts", "")
        .replace(".js", "");

    match category {
        // Smart surface: must be imported by entry point or router
        "smart" => {
            let mut imported_by_entry_or_router = false;
            for cf in all_files {
                let cb = match cf.split('/').next_back() {
                    Some(b) => b,
                    None => "",
                };
                let cf_suffix = get_surface_suffix(cb);
                // Entry point or router
                if cb.starts_with("cli_")
                    || cb.starts_with("mcp_")
                    || cf_suffix == "entry"
                    || cf_suffix == "router"
                {
                    if let Ok(c) = std::fs::read_to_string(cf) {
                        if c.contains(&stem) {
                            imported_by_entry_or_router = true;
                            break;
                        }
                    }
                }
            }
            OrphanIndicatorResult::new(
                !imported_by_entry_or_router,
                AesOrphanViolation::SurfaceOrphan {
                    category: "smart",
                    stem: stem.clone(),
                    reason: Some(
                        format!(
                            "Smart surface '{}' not imported by any entry point or router.",
                            stem
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            )
        }
        // Utility surface: must be imported by smart surface
        "utility" => {
            let mut imported_by_smart = false;
            for cf in all_files {
                let cb = match cf.split('/').next_back() {
                    Some(b) => b,
                    None => "",
                };
                let cf_suffix = get_surface_suffix(cb);
                if surface_category(&cf_suffix) == "smart" {
                    if let Ok(c) = std::fs::read_to_string(cf) {
                        if c.contains(&stem) {
                            imported_by_smart = true;
                            break;
                        }
                    }
                }
            }
            OrphanIndicatorResult::new(
                !imported_by_smart,
                AesOrphanViolation::SurfaceOrphan {
                    category: "utility",
                    stem: stem.clone(),
                    reason: Some(
                        format!(
                            "Utility surface '{}' not imported by any smart surface.",
                            stem
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            )
        }
        // Passive surface: must be imported by smart or utility surface
        "passive" => {
            let mut imported = false;
            for cf in all_files {
                let cb = match cf.split('/').next_back() {
                    Some(b) => b,
                    None => "",
                };
                let cf_suffix = get_surface_suffix(cb);
                let cat = surface_category(&cf_suffix);
                if cat == "smart" || cat == "utility" {
                    if let Ok(c) = std::fs::read_to_string(cf) {
                        if c.contains(&stem) {
                            imported = true;
                            break;
                        }
                    }
                }
            }
            OrphanIndicatorResult::new(
                !imported,
                AesOrphanViolation::SurfaceOrphan {
                    category: "passive",
                    stem: stem.clone(),
                    reason: Some(
                        format!(
                            "Passive surface '{}' not imported by any smart or utility surface.",
                            stem
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            )
        }
        _ => OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    }
}

pub fn check_surfaces_orphan(
    fp: &str,
    all_files: &[String],
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    let fp_vo = match FilePath::new(fp.to_string()) {
        Ok(p) => p,
        Err(_) => return,
    };
    let result = is_surface_orphan_raw(
        &fp_vo,
        all_files,
    );
    if result.is_orphan {
        violations.push(crate::agent_orphan_orchestrator::mk_orphan_result(
            fp,
            &result.reason,
            result.severity,
            "AES506",
        ));
    }
}
