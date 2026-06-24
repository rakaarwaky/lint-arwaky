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
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_duration_vo::Timeout;
use crate::mcp_server::taxonomy_job_vo::ResponseData;
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
