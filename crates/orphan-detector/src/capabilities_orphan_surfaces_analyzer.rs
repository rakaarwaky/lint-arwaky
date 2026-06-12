// PURPOSE: SurfacesOrphanAnalyzer — ISurfacesOrphanProtocol for orphan surface detection
// Surface orphan detection per category:
// - Smart surface (_command, _controller, _page, _entry): must be imported by entry or router
// - Utility surface (_hook, _store, _action, _screen, _router): must be imported by smart surface
// - Passive surface (_component, _view, _layout): must be imported by smart or utility surface
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::orphan_detector::contract_orphan_protocol::ISurfacesOrphanProtocol;
use shared::output_report::taxonomy_severity_vo::Severity;
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
    let orphan = !alive.contains(&f.value().to_string());
    OrphanIndicatorResult::new(orphan, "Surface is unreachable.".into(), Severity::MEDIUM)
}

pub fn is_surface_orphan_raw(f: &FilePath, all_files: &[String]) -> OrphanIndicatorResult {
    let fp = f.value();
    let basename = fp.split('/').next_back().unwrap_or("");
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
                let cb = cf.split('/').next_back().unwrap_or("");
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
                format!(
                    "Smart surface '{}' not imported by any entry point or router.",
                    stem
                ),
                Severity::MEDIUM,
            )
        }
        // Utility surface: must be imported by smart surface
        "utility" => {
            let mut imported_by_smart = false;
            for cf in all_files {
                let cb = cf.split('/').next_back().unwrap_or("");
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
                format!(
                    "Utility surface '{}' not imported by any smart surface.",
                    stem
                ),
                Severity::MEDIUM,
            )
        }
        // Passive surface: must be imported by smart or utility surface
        "passive" => {
            let mut imported = false;
            for cf in all_files {
                let cb = cf.split('/').next_back().unwrap_or("");
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
                format!(
                    "Passive surface '{}' not imported by any smart or utility surface.",
                    stem
                ),
                Severity::MEDIUM,
            )
        }
        _ => OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    }
}

pub fn check_surfaces_orphan(
    fp: &str,
    _ctx: &crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext,
    violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
) {
    let all_files: Vec<String> = Vec::new(); // TODO: pass actual file list
    let result = is_surface_orphan_raw(
        &FilePath::new(fp.to_string()).unwrap_or_default(),
        &all_files,
    );
    if result.is_orphan {
        violations.push(crate::orphan_detector::mk_orphan_result(
            fp,
            &result.reason,
            result.severity,
        ));
    }
}
