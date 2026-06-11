// PURPOSE: IFixRunnerAggregate — contract for auto-fix feature orchestrator (code-analysis layer)
use async_trait::async_trait;
use auto_fix::taxonomy_fix_vo::FixResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[async_trait]
pub trait IFixRunnerAggregate: Send + Sync {
    async fn run_fix(&self, target: &FilePath, dry_run: bool) -> FixResult;
    fn name(&self) -> &str;
}