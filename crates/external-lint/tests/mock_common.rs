//! Test helpers for `external-lint` integration tests.

use shared_common::taxonomy_path_vo::FilePath;

pub fn fp(path: &str) -> FilePath {
    FilePath::new(path.to_string()).expect("valid file path in test")
}
