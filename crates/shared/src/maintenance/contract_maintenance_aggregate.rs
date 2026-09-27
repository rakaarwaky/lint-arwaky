// PURPOSE: IMaintenanceAggregate — aggregate trait for maintenance operations (stats, doctor, clean, update, cancel)
use crate::maintenance::taxonomy_maintenance_request::MaintenanceRequest;
use crate::maintenance::taxonomy_maintenance_response::MaintenanceResponse;

pub trait IMaintenanceAggregate: Send + Sync {
    fn execute(&self, request: MaintenanceRequest) -> MaintenanceResponse;
}
