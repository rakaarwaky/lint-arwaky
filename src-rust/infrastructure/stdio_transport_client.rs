/// stdio_transport_client — Direct subprocess execution transport.
use crate::contract::command_executor_port::ICommandExecutorPort;
use crate::taxonomy::{
    AccessDeniedError, ActionArgs, ActionName, ActualValue, AdapterClassMap, AdapterEntry,
    AdapterError, AdapterMetadata, AdapterMetadataList, AdapterName, AdapterNameList,
    AdapterRegistered, AdapterStatus, AgentStatus, AgentStatusVO,
};
use crate::taxonomy::{
    AggregatedResults, AppConfig, ArchitectureConfig, ArchitectureRule, BooleanVO, CallChainError,
    CallChainList, CapabilityReference, CapabilityReferenceList, CapabilityRoutingContext, Cause,
    ClassDefinitionMap, ClassFileMap, ClassMethodsVO, ClassNameVO,
};
use crate::taxonomy::{
    ClassPath, ClassUsageItem, ClassUsageItemList, ClassUsageMap, ColumnNumber, CommandArgs,
    CommandMetadataVO, ComplianceStatus, ConfigError, ConfigKey, Constraint, ContentString, Count,
    CustomMessageVO, DataFlowList,
};
use crate::taxonomy::{
    DescriptionVO, DirectoryPath, DiscoveryError, DoctorResultVO, EnvContentVO, ErrorCode,
    ErrorMessage, ExitCode, ExpectedValue, FieldName, FileContentVO, FileDefinitionMap, FileFormat,
    FilePath,
};
use crate::taxonomy::{
    FilePathList, FileSystemError, FixApplied, FixResult, GitDiffResultVO, GitHookError, GitRef,
    GovernanceReport, GraphAnalysisContext, HookInstalled, HookRemoved, Identity, ImportGraph,
    ImportInfo, ImportInfoList,
};
use crate::taxonomy::{
    ImportNameList, InboundLinkMap, InheritanceMap, IntoPatternListValues, JobError, JobId,
    JobIdList, JobStatus, LayerDefinition, LayerMapVO, LayerNameVO, LegacyLayerRule,
    LegacyLayerRuleList, LineContentList, LineContentVO,
};
use crate::taxonomy::{
    LineNumber, LintMessage, LintResult, LintResultList, LintStatusActionArgs,
    LinterOperationError, Location, LocationList, LogOutput, MaintenanceStatsVO,
    MandatoryImportRuleVO, McpConfigVO, MetadataVO, MetricsError, ModuleName,
};
use crate::taxonomy::{
    ModuleToFileMap, NameVariants, NamingConfig, NamingError, OrphanIndicatorResult,
    PathNotFoundError, PatternList, PluginError, PluginGroup, Position, PrimitiveTypeList,
    PrimitiveTypeName, PrimitiveViolation, PrimitiveViolationList, ProjectConfig,
};
use crate::taxonomy::{
    ProjectResult, ReachabilityResult, RegistrationError, RenamedFile, RenamedFileList,
    ResponseData, ResponseDataList, ScanCompleted, ScanError, ScanFailed, ScanStarted, ScopeBounds,
    ScopeRef, ScopeResolutionError, Score,
};
use crate::taxonomy::{
    SemanticError, Severity, SourceParserError, StdError, StdOutput, SuccessStatus, SuffixPolicyVO,
    SuffixVO, Suggestion, SymbolName, SymbolNameList, SyntaxErrorVO, Thresholds, Timeout,
    Timestamp,
};
use crate::taxonomy::{
    TransportEndpoint, TransportError, TransportProtocol, TransportUrlVO, ValidationError,
    ViolationConstraint, WatchEventError, WatchResult, WatchServiceError, WatchSubscriptionError,
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Duration;
use tokio::process::Command;

pub struct StdioClient {
    timeout: Duration,
}

impl StdioClient {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

#[async_trait]
impl ICommandExecutorPort for StdioClient {
    async fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Duration>,
    ) -> anyhow::Result<ResponseData> {
        let timeout_val = timeout.unwrap_or(self.timeout);
        let cmd_list: Vec<&str> = command.values.iter().map(|s| s.as_ref()).collect();
        if cmd_list.is_empty() {
            anyhow::bail!("Empty command");
        }
        let mut cmd = Command::new(cmd_list[0]);
        if cmd_list.len() > 1 {
            cmd.args(&cmd_list[1..]);
        }
        cmd.current_dir(&working_dir.value)
            .env("PYTHONUNBUFFERED", "1");
        cmd.kill_on_drop(true);

        let result = tokio::time::timeout(timeout_val, cmd.output()).await;
        match result {
            Ok(Ok(output)) => {
                let mut meta_map = HashMap::new();
                meta_map.insert(
                    "protocol".to_string(),
                    serde_json::Value::String("Stdio".to_string()),
                );

                Ok(ResponseData {
                    value: Some(serde_json::Value::Null),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    returncode: output.status.code().unwrap_or(-1) as i64,
                    metadata: meta_map,
                })
            }
            Ok(Err(e)) => anyhow::bail!("Command execution failed: {}", e),
            Err(_) => {
                anyhow::bail!("Command timed out after {}s", timeout_val.as_secs())
            }
        }
    }

    async fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}
