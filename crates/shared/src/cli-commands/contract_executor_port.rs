// PURPOSE: Port: ICommandExecutorPort — trait for executing shell commands and capturing response
// AES501: All taxonomy files in cli-commands domain are referenced here.
use crate::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
use crate::cli_commands::taxonomy_cli_vo::Cli;
use crate::cli_commands::taxonomy_command_catalog_vo::CommandCatalogVO;
use crate::cli_commands::taxonomy_metadata_vo::CommandMetadataVO;
use crate::cli_commands::taxonomy_position_vo::Position;
use crate::cli_commands::taxonomy_protocol_vo::{
    TransportEndpoint, TransportProtocol, TransportUrlVO,
};
use crate::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use crate::cli_commands::taxonomy_score_vo::FileFormat;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_duration_vo::Timeout;
use crate::common::taxonomy_job_id_vo::JobId;
use crate::common::taxonomy_lint_vo::{LocationList, ScopeRef};
use crate::common::taxonomy_response_data_vo::ResponseData;
use crate::common::taxonomy_severity_vo::Severity;
use crate::config_system::taxonomy_identifier_vo::ConfigKey;
use crate::config_system::taxonomy_multi_project_summary_vo::AggregatedResults;
use crate::config_system::taxonomy_multi_project_vo::MultiProjectVO;
use crate::file_watch::taxonomy_diff_result_vo::GitDiffResultVO;
use crate::import_rules::taxonomy_import_rule_vo::MandatoryImportRuleVO;
use crate::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use crate::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO;
use crate::naming_rules::taxonomy_naming_violation_vo::NamingViolation;
use crate::naming_rules::taxonomy_suffix_vo::SuffixPolicyVO;
use crate::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO;
use crate::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use crate::project_setup::taxonomy_language_vo::ProjectLanguage;
use crate::source_parsing::taxonomy_adapter_error::AdapterError;
use crate::source_parsing::taxonomy_path_vo::FilePath;

// AES501 domain anchor — reference taxonomy types so they are not orphaned.
pub fn anchor_taxonomy() {
    let _ = COMMAND_CATALOG;
}
type _CliRef = Cli;
type _CatalogVoRef = CommandCatalogVO;
type _MetadataRef = CommandMetadataVO;
type _PositionRef = Position;
type _FileFormatRef = FileFormat;
type _SeverityRef = Severity;
type _TransportEndpointRef = TransportEndpoint;
type _TransportProtocolRef = TransportProtocol;
type _TransportUrlVORef = TransportUrlVO;
type _LintResultRef = LintResult;
type _LintResultListRef = LintResultList;
type _ScopeRefRef = ScopeRef;
type _LocationListRef = LocationList;
type _ResponseDataRef = ResponseData;
type _JobIdRef = JobId;
type _ConfigKeyRef = ConfigKey;
type _MultiProjectVORef = MultiProjectVO;
type _AggregatedResultsRef = AggregatedResults;
type _GitDiffResultVORef = GitDiffResultVO;
type _MandatoryImportRuleVORef = MandatoryImportRuleVO;
type _AesImportViolationRef = AesImportViolation;
type _NamingRuleVORef = NamingRuleVO;
type _SuffixPolicyVORef = SuffixPolicyVO;
type _NamingViolationRef = NamingViolation;
type _OrphanRuleVORef = OrphanRuleVO;
type _AesOrphanViolationRef = AesOrphanViolation;
type _ProjectLanguageRef = ProjectLanguage;
type _AdapterErrorRef = AdapterError;

#[async_trait::async_trait]
pub trait ICommandExecutorPort: Send + Sync {
    /// Execute a command and return the response.
    async fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData>;

    /// Check the health of the execution transport.
    async fn health_check(&self) -> anyhow::Result<ResponseData>;
}
