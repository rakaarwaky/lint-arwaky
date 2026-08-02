// PURPOSE: Maintenance — CLI thin wrapper
// Calls dispatcher for maintenance business logic, only adds CLI output
use shared::common::ExitCode;
use shared::common::FilePath;
use shared::maintenance::MaintenanceCommandsAggregate;
use std::sync::Arc;

pub fn handle_doctor(maintenance: Arc<dyn MaintenanceCommandsAggregate>) -> ExitCode {
    dispatcher::surface_maintenance_action::handle_doctor(maintenance)
}

pub fn handle_security(
    maintenance: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    dispatcher::surface_maintenance_action::handle_security(maintenance, path)
}

pub fn handle_dependencies(
    maintenance: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    dispatcher::surface_maintenance_action::handle_dependencies(maintenance, path)
}
