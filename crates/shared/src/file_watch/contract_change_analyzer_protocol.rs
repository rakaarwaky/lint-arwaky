// PURPOSE: IChangeAnalyzerProtocol — protocol for watch event change analysis
use crate::file_watch::taxonomy_watch_event_vo::WatchEvent;

/// Protocol for analysing file-system watch events.
///
/// Implementations group, filter, and prioritise raw [`WatchEvent`]s so
/// that only lintable changes trigger re-scans. The `is_lintable` method
/// is a static check that can be called without an instance.
pub trait IChangeAnalyzerProtocol: Send + Sync {
    fn analyze(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent>;
    fn is_lintable(path: &str) -> bool;
    fn filter_lintable(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent>;
}
