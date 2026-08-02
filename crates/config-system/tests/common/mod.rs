#![allow(dead_code)]

use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

pub fn make_fs() -> Arc<dyn IFilesystemAggregate> {
    filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator()
}

pub fn make_container() -> ConfigContainer {
    ConfigContainer::new(make_fs())
}
