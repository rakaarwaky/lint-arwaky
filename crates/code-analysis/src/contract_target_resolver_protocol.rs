// PURPOSE: ITargetResolverProtocol — protocol for project target resolution, file walking, and lint path score queries.

use crate::output_report::taxonomy_result_vo::LintResult;
use std::path::{Path, PathBuf};

pub trait ITargetResolverProtocol: Send + Sync {
    fn resolve_target(&self, path: Option<String>) -> String;
    fn walk_rs_files(&self, dir: &Path, cb: &mut dyn FnMut(PathBuf));
    fn lint_path(&self, path: &str) -> Vec<LintResult>;
    fn compute_score(&self, results: &[LintResult]) -> f64;
}
