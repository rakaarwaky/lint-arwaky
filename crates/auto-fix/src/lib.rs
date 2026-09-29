pub use shared::auto_fix::IFixAggregate;
pub use shared::auto_fix::{
    IBypassFixProtocol, ISymbolRenameProtocol, IUnusedImportFixProtocol, IViolationReportProtocol,
};

pub mod agent_fix_orchestrator;
pub use agent_fix_orchestrator::FixOrchestrator;

pub mod capabilities_bypass_fix;
pub use capabilities_bypass_fix::BypassFix;

pub mod capabilities_symbol_rename;
pub use capabilities_symbol_rename::SymbolRename;

pub mod capabilities_unused_import_fix;
pub use capabilities_unused_import_fix::UnusedImportFix;

pub mod capabilities_violation_report;
pub use capabilities_violation_report::ViolationReport;

pub mod root_auto_fix_container;
