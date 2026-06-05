use crate::contract::file_system_port::IFileSystemPort;
use crate::contract::command_executor_port::ICommandExecutorPort;
use crate::contract::path_normalization_port::IPathNormalizationPort;
use crate::contract::source_parser_port::ISourceParserPort;
use crate::contract::linter_adapter_port::ILinterAdapterPort;
use crate::contract::architecture_lint_protocol::IArchLintProtocol;
use std::sync::Arc;

pub trait ServiceContainerAggregate: Send + Sync {
    fn file_system(&self) -> Arc<dyn IFileSystemPort>;
    fn command_executor(&self) -> Arc<dyn ICommandExecutorPort>;
    fn path_normalization(&self) -> Arc<dyn IPathNormalizationPort>;
    fn source_parser(&self) -> Arc<dyn ISourceParserPort>;
    fn linter_adapter(&self, name: &str) -> Option<Arc<dyn ILinterAdapterPort>>;
    fn get_architecture_linter(&self) -> Option<Arc<dyn IArchLintProtocol>>;
}
