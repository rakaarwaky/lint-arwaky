#![allow(dead_code)]

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

pub fn make_fs() -> Arc<dyn IFilesystemAggregate> {
    filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator()
}

pub fn make_container() -> MaintenanceContainer {
    MaintenanceContainer::new(make_fs())
}
