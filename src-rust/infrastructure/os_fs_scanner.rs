/// Implementation of IFileSystemPort using standard std::fs.
use crate::contract::file_system_port::IFileSystemPort;
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
use async_trait::async_trait;
use std::fs;
use std::path::{Path, PathBuf};

pub struct OSFileSystemAdapter;

impl OSFileSystemAdapter {
    pub fn new() -> Self {
        Self
    }

    fn walk_recursive(&self, dir: &Path, ignored: &[String], results: &mut Vec<FilePath>) {
        if dir.is_file() {
            results.push(FilePath::new(dir.to_string_lossy().to_string()));
            return;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if ignored.contains(&name.to_string()) {
                    continue;
                }

                if path.is_dir() {
                    self.walk_recursive(&path, ignored, results);
                } else {
                    results.push(FilePath::new(path.to_string_lossy().to_string()));
                }
            }
        }
    }
}

#[async_trait]
impl IFileSystemPort for OSFileSystemAdapter {
    async fn walk(&self, path: &FilePath, ignored_patterns: Option<&PatternList>) -> Vec<FilePath> {
        let root = Path::new(&path.value);
        let ignored = ignored_patterns
            .map(|p| p.values.clone())
            .unwrap_or_default();
        let mut results = Vec::new();
        self.walk_recursive(root, &ignored, &mut results);
        results
    }

    async fn is_directory(&self, path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(BooleanVO::new(Path::new(&path.value).is_dir()))
    }

    async fn is_file(&self, path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(BooleanVO::new(Path::new(&path.value).is_file()))
    }

    async fn get_relative_path(&self, path: &FilePath, start: &FilePath) -> FilePath {
        let p = Path::new(&path.value);
        let s = Path::new(&start.value);
        match p.strip_prefix(s) {
            Ok(rel) => FilePath::new(rel.to_string_lossy().to_string()),
            Err(_) => path.clone(),
        }
    }

    async fn read_text(&self, path: &FilePath) -> Result<ContentString, FileSystemError> {
        self.read_file(path).await
    }

    async fn get_line_count(&self, path: &FilePath) -> Count {
        if let Ok(content) = fs::read_to_string(&path.value) {
            Count::new(content.lines().count() as i64)
        } else {
            Count::new(0)
        }
    }

    async fn exists(&self, path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(BooleanVO::new(Path::new(&path.value).exists()))
    }

    async fn get_parent(&self, path: &FilePath) -> FilePath {
        let p = Path::new(&path.value);
        match p.parent() {
            Some(parent) => FilePath::new(parent.to_string_lossy().to_string()),
            None => path.clone(),
        }
    }

    async fn write_text(
        &self,
        path: &FilePath,
        content: &ContentString,
        _mode: Option<&Identity>,
    ) -> Result<SuccessStatus, FileSystemError> {
        match fs::write(&path.value, &content.value) {
            Ok(_) => Ok(SuccessStatus::new(BooleanVO::new(true))),
            Err(e) => Err(FileSystemError {
                path: path.clone(),
                message: ErrorMessage::new(e.to_string()),
                ..Default::default()
            }),
        }
    }

    async fn glob(&self, pattern: &Identity) -> Vec<FilePath> {
        // Simple mock glob
        vec![]
    }

    async fn get_cwd(&self) -> FilePath {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        FilePath::new(cwd.to_string_lossy().to_string())
    }

    async fn get_basename(&self, path: &FilePath) -> Identity {
        let p = Path::new(&path.value);
        Identity::new(
            p.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string(),
        )
    }

    async fn path_join(&self, parts: &[Identity]) -> FilePath {
        let mut path = PathBuf::new();
        for part in parts {
            path.push(&part.value);
        }
        FilePath::new(path.to_string_lossy().to_string())
    }

    async fn read_file(&self, path: &FilePath) -> Result<ContentString, FileSystemError> {
        match fs::read_to_string(&path.value) {
            Ok(content) => Ok(ContentString::new(content)),
            Err(e) => Err(FileSystemError {
                path: path.clone(),
                message: ErrorMessage::new(e.to_string()),
                ..Default::default()
            }),
        }
    }
}
