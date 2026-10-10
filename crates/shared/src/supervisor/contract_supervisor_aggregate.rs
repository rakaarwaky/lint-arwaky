// PURPOSE: ISupervisorAggregate — aggregate trait for the supervisor workflow

use crate::taxonomy_supervisor_request::SupervisorRequest;
use crate::taxonomy_supervisor_response::SupervisorResponse;

pub trait ISupervisorAggregate: Send + Sync {
    fn execute(&self, request: SupervisorRequest) -> SupervisorResponse;
}
