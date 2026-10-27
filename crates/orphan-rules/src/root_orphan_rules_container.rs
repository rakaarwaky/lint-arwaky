// PURPOSE: Root-layer composition container for the orphan-rules feature.

use crate::root_orphan_detector_container::OrphanContainer;
use shared_config_system::{ArchitectureConfig, IConfigOrchestratorAggregate};
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol;

use std::sync::Arc;

/// The AES-named root container for the `orphan_rules` feature module. It owns
/// the same composition wiring as `root_orphan_detector_container` — the
/// canonical name is the module name, `orphan_rules`.
pub struct OrphanRulesContainer {
    inner: OrphanContainer,
}

impl OrphanRulesContainer {
    pub fn new(
        filesystem: Arc<dyn IFilesystemAggregate>,
        workspace: Arc<dyn IWorkspaceProtocol>,
    ) -> Self {
        Self {
            inner: OrphanContainer::new(filesystem, workspace),
        }
    }

    pub fn new_with_config(
        config: ArchitectureConfig,
        filesystem: Arc<dyn IFilesystemAggregate>,
        workspace: Arc<dyn IWorkspaceProtocol>,
    ) -> Self {
        Self {
            inner: OrphanContainer::new_with_config(config, filesystem, workspace),
        }
    }

    /// Create from config orchestrator — the canonical way per AES architecture.
    pub fn from_orchestrator(
        orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
        project_root: &str,
        filesystem: Arc<dyn IFilesystemAggregate>,
        workspace: Arc<dyn IWorkspaceProtocol>,
    ) -> Self {
        Self {
            inner: OrphanContainer::from_orchestrator(
                orchestrator,
                project_root,
                filesystem,
                workspace,
            ),
        }
    }

    pub fn analyzer(&self) -> Arc<dyn shared_orphan_rules::IOrphanAggregate> {
        self.inner.analyzer()
    }
}
