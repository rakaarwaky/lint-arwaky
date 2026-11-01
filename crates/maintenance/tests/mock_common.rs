#![allow(dead_code)]

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use std::sync::Arc;

pub fn make_fs() -> Arc<dyn IFilesystemAggregate> {
    filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator()
}

pub fn make_io() -> Arc<dyn IFileSystemIOProtocol> {
    filesystem::root_filesystem_container::FilesystemContainer::new().io()
}

pub fn make_container() -> MaintenanceContainer {
    MaintenanceContainer::new(make_fs(), make_io())
}
