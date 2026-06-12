// PURPOSE: WatchServiceError — structured error type for file watch service failures
use crate::source_parsing::taxonomy_path_vo::FilePath;

define_error! {
    pub struct WatchServiceError {
        pub path: FilePath,
    }
    display("Watch Error", path: " on {}")
}

define_wrapper! {
    pub struct WatchSubscriptionError {
        pub base: WatchServiceError,
    }
}

define_wrapper! {
    pub struct WatchEventError {
        pub base: WatchServiceError,
    }
}
