// PURPOSE: IMcpSchemaCheckerProtocol — port trait for MCP tool schema checks.

use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IMcpSchemaCheckerProtocol: Send + Sync {
    fn check_mcp_tool_schema(
        &self,
        files: &[FilePath],
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
