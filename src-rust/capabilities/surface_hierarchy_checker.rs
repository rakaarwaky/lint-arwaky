// surface_hierarchy_checker — AES018/AES019 for surface hierarchy enforcement.
//
// AES018 SURFACE_HIERARCHY_VIOLATION:
// A file that is NOT an __init__.py barrel in the surfaces layer is not
// imported from the layer __init__.py — meaning it is unreachable from the
// surface entry point.
//
// AES019 PASSIVE_SURFACE_VIOLATION:
// A surface file contains complex domain logic (many public methods, deep
// control flow) instead of acting as a thin pass-through to the agent layer.
// Surfaces must be declarative/passive — I/O parsing + delegation only.

use crate::taxonomy::{AccessDeniedError,
ActionArgs,
ActionName,
ActualValue,
AdapterClassMap,
AdapterEntry,
AdapterError,
AdapterMetadata,
AdapterMetadataList,
AdapterName,
AdapterNameList,
AdapterRegistered,
AdapterStatus,
AgentStatus,
AgentStatusVO};

use crate::taxonomy::{AggregatedResults,
AppConfig,
ArchitectureConfig,
ArchitectureRule,
BooleanVO,
CallChainError,
CallChainList,
CapabilityReference,
CapabilityReferenceList,
CapabilityRoutingContext,
Cause,
ClassDefinitionMap,
ClassFileMap,
ClassMethodsVO,
ClassNameVO};

use crate::taxonomy::{ClassPath,
ClassUsageItem,
ClassUsageItemList,
ClassUsageMap,
ColumnNumber,
CommandArgs,
CommandMetadataVO,
ComplianceStatus,
ConfigError,
ConfigKey,
Constraint,
ContentString,
Count,
CustomMessageVO,
DataFlowList};

use crate::taxonomy::{DescriptionVO,
DirectoryPath,
DiscoveryError,
DoctorResultVO,
Duration,
EnvContentVO,
ErrorCode,
ErrorMessage,
ExitCode,
ExpectedValue,
FieldName,
FileContentVO,
FileDefinitionMap,
FileFormat,
FilePath};

use crate::taxonomy::{FilePathList,
FileSystemError,
FixApplied,
FixResult,
GitDiffResultVO,
GitHookError,
GitRef,
GovernanceReport,
GraphAnalysisContext,
HookInstalled,
HookRemoved,
Identity,
ImportGraph,
ImportInfo,
ImportInfoList};

use crate::taxonomy::{ImportNameList,
InboundLinkMap,
InheritanceMap,
IntoPatternListValues,
JobError,
JobId,
JobIdList,
JobStatus,
LayerDefinition,
LayerMapVO,
LayerNameVO,
LegacyLayerRule,
LegacyLayerRuleList,
LineContentList,
LineContentVO};

use crate::taxonomy::{LineNumber,
LintMessage,
LintResult,
LintResultList,
LintStatusActionArgs,
LinterOperationError,
Location,
LocationList,
LogOutput,
MaintenanceStatsVO,
MandatoryImportRuleVO,
McpConfigVO,
MetadataVO,
MetricsError,
ModuleName};

use crate::taxonomy::{ModuleToFileMap,
NameVariants,
NamingConfig,
NamingError,
OrphanIndicatorResult,
PathNotFoundError,
PatternList,
PluginError,
PluginGroup,
Position,
PrimitiveTypeList,
PrimitiveTypeName,
PrimitiveViolation,
PrimitiveViolationList,
ProjectConfig};

use crate::taxonomy::{ProjectResult,
ReachabilityResult,
RegistrationError,
RenamedFile,
RenamedFileList,
ResponseData,
ResponseDataList,
ScanCompleted,
ScanError,
ScanFailed,
ScanStarted,
ScopeBounds,
ScopeRef,
ScopeResolutionError,
Score};

use crate::taxonomy::{SemanticError,
Severity,
SourceParserError,
StdError,
StdOutput,
SuccessStatus,
SuffixPolicyVO,
SuffixVO,
Suggestion,
SymbolName,
SymbolNameList,
SyntaxErrorVO,
Thresholds,
Timeout,
Timestamp};

use crate::taxonomy::{TransportEndpoint,
TransportError,
TransportProtocol,
TransportUrlVO,
ValidationError,
ViolationConstraint,
WatchEventError,
WatchResult,
WatchServiceError,
WatchSubscriptionError};
use once_cell::sync::Lazy;
use regex::Regex;

// Regex: detect Python function/method definitions inside a class
static PY_METHOD_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:async\s+)?def\s+(\w+)\s*\(").unwrap()
});

// Regex: detect class definitions
static PY_CLASS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^class\s+(\w+)").unwrap()
});

// Regex: detect if statements for nesting depth
static IF_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*if\s+").unwrap()
});

/// AES018 + AES019 — surface barrel wiring and passivity checks.
pub struct SurfaceHierarchyChecker;

// Thresholds for AES019
const MAX_PUBLIC_METHODS: usize = 10;
const MAX_FUNCTION_BODY_LINES: i64 = 80;
const MAX_IF_DEPTH: usize = 3;

impl SurfaceHierarchyChecker {
    pub fn new() -> Self {
        Self
    }

    /// Main entry point — run AES018 (barrel wiring) and AES019 (passive surface).
    pub fn check_surface_hierarchy(
        &self,
        files: &[FilePath],
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in files {
            if !is_in_surfaces(f) {
                continue;
            }
            if is_init(f) {
                continue;
            }

            // AES018: check if file is wired in barrel
            if !is_wired(f) {
                let desc = format!(
                    "AES018 SURFACE_HIERARCHY_VIOLATION: Surface file '{}' is not imported from the layer barrel.\n\
                     WHY? All surface files must be reachable through __init__.py to maintain a clear entry-point hierarchy.\n\
                     FIX: Add 'from .{} import ...' to {}/__init__.py, or delete if unused.",
                    f,
                    stem(f),
                    directory(f)
                );
                results.append(LintResult {
                    file: f.clone(),
                    line: LineNumber::new(1),
                    column: ColumnNumber::new(1),
                    code: ErrorCode::new("AES018").unwrap(),
                    message: LintMessage::new(desc),
                    source: Some(AdapterName::new("surface_hierarchy").unwrap()),
                    severity: Severity::CRITICAL,
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            }

            // AES019: check if file is passive
            self._check_passive(f, results);
        }
    }

    /// Check if a surface file is passive (thin I/O boundary).
    fn _check_passive(&self, f: &FilePath, results: &mut LintResultList) {
        let content = match std::fs::read_to_string(f.to_string()) {
            Ok(c) => c,
            Err(_) => return,
        };

        let lines: Vec<&str> = content.lines().collect();
        let mut violations: Vec<String> = Vec::new();

        // Find classes in the file and check their methods
        for (i, raw_line) in lines.iter().enumerate() {
            let stripped = raw_line.trim();
            if let Some(cap) = PY_CLASS_RE.captures(stripped) {
                let class_name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                let class_start = i;
                let indent = raw_line.len() - raw_line.trim_start().len();

                // Collect public methods in this class
                let mut pub_methods: Vec<(String, usize, Option<usize>)> = Vec::new();

                for j in (i + 1)..lines.len() {
                    let method_line = lines[j];
                    if method_line.trim().is_empty() {
                        continue;
                    }
                    let m_indent = method_line.len() - method_line.trim_start().len();

                    // If indent <= class indent, we've left the class
                    if m_indent <= indent && !method_line.trim().is_empty() {
                        break;
                    }

                    if let Some(mcap) = PY_METHOD_RE.captures(method_line.trim()) {
                        let method_name = mcap.get(1).map(|m| m.as_str()).unwrap_or("");
                        // Public methods don't start with underscore
                        if !method_name.starts_with('_') {
                            // Estimate end line
                            let mut end_line = lines.len();
                            for k in (j + 1)..lines.len() {
                                let next = lines[k];
                                if !next.trim().is_empty() {
                                    let n_indent = next.len() - next.trim_start().len();
                                    if n_indent <= m_indent {
                                        end_line = k;
                                        break;
                                    }
                                }
                            }
                            pub_methods.push((method_name.to_string(), j + 1, Some(end_line)));
                        }
                    }
                }

                self._check_methods_too_public(class_name, &pub_methods, &mut violations);
                self._check_method_lengths(class_name, &lines, &pub_methods, &mut violations);
                self._check_method_nesting(class_name, &lines, &pub_methods, &mut violations);
            }
        }

        if !violations.is_empty() {
            self._report_aes019(f, violations, results);
        }
    }

    // -- AES019 sub-checks ---------------------------------------------------

    /// AES019: too many public methods in a surface class.
    fn _check_methods_too_public(
        &self,
        class_name: &str,
        pub_methods: &[(String, usize, Option<usize>)],
        violations: &mut Vec<String>,
    ) {
        if pub_methods.len() > MAX_PUBLIC_METHODS {
            violations.push(format!(
                "Class '{}' has {} public methods (max {})",
                class_name,
                pub_methods.len(),
                MAX_PUBLIC_METHODS
            ));
        }
    }

    /// AES019: method body exceeds line limit.
    fn _check_method_lengths(
        &self,
        class_name: &str,
        _lines: &[&str],
        pub_methods: &[(String, usize, Option<usize>)],
        violations: &mut Vec<String>,
    ) {
        for (method_name, start, end) in pub_methods {
            if let Some(end_line) = end {
                let body_len = (*end_line as i64) - (*start as i64);
                if body_len > MAX_FUNCTION_BODY_LINES {
                    violations.push(format!(
                        "Method '{}.{}' is {} lines (max {})",
                        class_name, method_name, body_len, MAX_FUNCTION_BODY_LINES
                    ));
                }
            }
        }
    }

    /// AES019: method control-flow nesting exceeds limit.
    fn _check_method_nesting(
        &self,
        class_name: &str,
        lines: &[&str],
        pub_methods: &[(String, usize, Option<usize>)],
        violations: &mut Vec<String>,
    ) {
        for (method_name, start, end) in pub_methods {
            let end_line = end.unwrap_or(lines.len());
            let mut max_depth: usize = 0;

            for i in *start..end_line {
                if i >= lines.len() {
                    break;
                }
                let line = lines[i];
                let trimmed = line.trim();

                // Count nesting by indentation increase relative to method body
                if IF_RE.is_match(trimmed) {
                    let indent = line.len() - line.trim_start().len();
                    // Simple heuristic: count leading whitespace / 4 as depth
                    let depth = indent / 4;
                    if depth > max_depth {
                        max_depth = depth;
                    }
                }
            }

            if max_depth > MAX_IF_DEPTH {
                violations.push(format!(
                    "Method '{}.{}' has deep control flow (if-nesting > {})",
                    class_name, method_name, MAX_IF_DEPTH
                ));
            }
        }
    }

    /// Append a single AES019 result to the results list.
    fn _report_aes019(
        &self,
        f: &FilePath,
        violations: Vec<String>,
        results: &mut LintResultList,
    ) {
        let detail: String = violations
            .iter()
            .map(|v| format!("  - {}", v))
            .collect::<Vec<_>>()
            .join("\n");

        results.append(LintResult {
            file: f.clone(),
            line: LineNumber::new(1),
            column: ColumnNumber::new(1),
            code: ErrorCode::new("AES019").unwrap(),
            message: LintMessage::new(format!(
                "AES019 PASSIVE_SURFACE_VIOLATION: Surface file '{}' contains active domain logic:\n{}\n\
                 WHY? Surfaces must be passive I/O boundaries. Business logic belongs in capabilities/agent layers.\n\
                 FIX: Move logic to a handler or orchestrator.",
                f, detail
            )),
            source: Some(AdapterName::new("surface_hierarchy").unwrap()),
            severity: Severity::CRITICAL,
            enclosing_scope: None,
            related_locations: LocationList::new(),
        });
    }
}

// --- helpers -----------------------------------------------------------------

/// Check if the file path is in a surfaces directory.
fn is_in_surfaces(f: &FilePath) -> bool {
    f.to_string().contains("/surfaces/") || f.to_string().ends_with("/surfaces")
}

/// Check if the file is a barrel/init file.
fn is_init(f: &FilePath) -> bool {
    let path_str = f.to_string();
    path_str.ends_with("__init__.py")
        || path_str.ends_with("mod.rs")
        || path_str.ends_with("index.ts")
        || path_str.ends_with("index.js")
}

/// Extract the file stem (filename without extension).
fn stem(f: &FilePath) -> String {
    let path_str = f.to_string();
    let basename = path_str.rsplit('/').next().unwrap_or(&path_str);
    basename.rsplit('.').next().unwrap_or(basename).to_string()
}

/// Get the directory portion of the file path.
fn directory(f: &FilePath) -> String {
    let path_str = f.to_string();
    path_str.rsplit('/').skip(1).next().unwrap_or(&path_str).to_string()
}

/// Check if a module stem is imported in its directory barrel.
fn is_wired(f: &FilePath) -> bool {
    let barrel_names = ["__init__.py", "mod.rs", "index.ts", "index.js"];
    let file_stem = stem(f);
    let dir = directory(f);

    for name in &barrel_names {
        let init_path = format!("{}/{}", dir, name);
        if let Ok(content) = std::fs::read_to_string(&init_path) {
            if content.contains(&format!("import {}", file_stem))
                || content.contains(&format!("from .{}", file_stem))
                || content.contains(&format!("\"{}\"", file_stem))
                || content.contains(&format!("'{}'", file_stem))
                || content.contains(&format!("mod {}", file_stem))
                || content.contains(&format!("use {}", file_stem))
            {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_surfaces() {
        let f = FilePath::new("src/surfaces/handler.py").unwrap();
        assert!(is_in_surfaces(&f));

        let f = FilePath::new("src/capabilities/checker.py").unwrap();
        assert!(!is_in_surfaces(&f));
    }

    #[test]
    fn test_is_init() {
        let f = FilePath::new("src/surfaces/__init__.py").unwrap();
        assert!(is_init(&f));

        let f = FilePath::new("src/surfaces/handler.py").unwrap();
        assert!(!is_init(&f));
    }

    #[test]
    fn test_stem() {
        let f = FilePath::new("src/surfaces/handler.py").unwrap();
        assert_eq!(stem(&f), "handler");
    }

    #[test]
    fn test_directory() {
        let f = FilePath::new("src/surfaces/handler.py").unwrap();
        assert_eq!(directory(&f), "src/surfaces");
    }
}
