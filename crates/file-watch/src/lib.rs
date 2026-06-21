// PURPOSE: Module declarations for file-watch (provider, analyzer, orchestrator, container)
pub mod infrastructure_notify_provider;
pub use infrastructure_notify_provider::NotifyWatchProvider;
pub mod capabilities_change_analyzer;
pub use capabilities_change_analyzer::ChangeAnalyzer;
pub mod agent_watch_orchestrator;
pub use agent_watch_orchestrator::WatchOrchestrator;
pub mod root_file_watch_container;
pub use root_file_watch_container::FileWatchContainer;
