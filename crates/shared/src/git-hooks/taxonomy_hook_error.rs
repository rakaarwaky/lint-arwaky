// PURPOSE: HookError — structured error type for git hook operation failures
use crate::source_parsing::taxonomy_path_vo::FilePath;

define_error! {
    pub struct GitHookError {
        pub path: FilePath,
    }
    display("Git Hook Error", path: " on {}")
}
