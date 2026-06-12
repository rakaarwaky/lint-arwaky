// PURPOSE: MetricsError — structured error type for metrics service failures
use crate::source_parsing::taxonomy_path_vo::FilePath;

define_error! {
    pub struct MetricsError {
        pub path: FilePath,
    }
    display("Metrics Error", path: " for {}")
}
