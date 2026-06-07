//! Protocol for project setup and configuration generation.
//!
//! Defines the inbound interface for generating environment
//! files and MCP server configurations for various editors.

use crate::taxonomy::{DirectoryPath, EnvContentVO, McpConfigVO};

pub trait ISetupManagementProtocol: Send + Sync {
    fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self) -> McpConfigVO;
    fn mcp_config_claude(&self) -> McpConfigVO;
    fn mcp_config_hermes(&self) -> McpConfigVO;
    fn mcp_config_vscode(&self) -> McpConfigVO;
}
