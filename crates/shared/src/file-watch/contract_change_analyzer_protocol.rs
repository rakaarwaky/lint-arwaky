// PURPOSE: IChangeAnalyzerProtocol — protocol for watch event change analysis
use crate::common::taxonomy_path_vo::FilePath;
use crate::file_watch::taxonomy_watch_event_vo::WatchEvent;

pub trait IChangeAnalyzerProtocol: Send + Sync {
    fn analyze(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent>;
    fn is_lintable(&self, path: &FilePath) -> bool;
    fn filter_lintable(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent>;
}
