use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared::common::contract_scanner_provider_port::IScannerProviderPort;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::git_hooks::contract_manager_port::IHookManagerPort;
use shared::git_hooks::taxonomy_hook_error::GitHookError;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use std::sync::Arc;

struct MockScanner;
struct MockHookAdapter;

impl IScannerProviderPort for MockScanner {
    fn scan(
        &self,
        _path: &FilePath,
        _extensions: &[String],
    ) -> Result<FilePathList, Box<dyn std::error::Error + Send + Sync>> {
        Ok(FilePathList::new(vec![]))
    }
}

impl IHookManagerPort for MockHookAdapter {
    fn install_pre_commit(&self, _path: &FilePath) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
}

#[test]
fn container_can_be_constructed() {
    let container = GitContainer::new(
        Arc::new(MockScanner) as Arc<dyn IScannerProviderPort>,
        Arc::new(MockHookAdapter) as Arc<dyn IHookManagerPort>,
    );
    let _ = container;
}

#[test]
fn container_default_constructs() {
    let container = GitContainer::new_default();
    let _ = container;
}

#[tokio::test]
async fn container_aggregate_is_accessible() {
    let container = GitContainer::new(
        Arc::new(MockScanner) as Arc<dyn IScannerProviderPort>,
        Arc::new(MockHookAdapter) as Arc<dyn IHookManagerPort>,
    );
    let aggregate = container.aggregate();
    let path = FilePath::new("test.rs".to_string()).unwrap_or_default();
    let results = aggregate.run_git_hooks_check(&path).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn container_aggregate_hook_operations() {
    use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
    let container = GitContainer::new(
        Arc::new(MockScanner) as Arc<dyn IScannerProviderPort>,
        Arc::new(MockHookAdapter) as Arc<dyn IHookManagerPort>,
    );
    let aggregate = container.aggregate();
    let path = FilePath::new("lint-arwaky".to_string()).unwrap_or_default();
    let result = aggregate.install_hook(&path).await;
    assert!(result.is_ok());
    let result = aggregate.uninstall_hook().await;
    assert!(result.is_ok());
}
