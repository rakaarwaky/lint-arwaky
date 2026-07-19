use std::sync::Arc;

use shared::<domain>::taxonomy_<name>_vo::<VO>;
use shared::<domain>::contract_<name>_aggregate::I<Name>Aggregate;

pub struct Surface<Name> {
    aggregate: Arc<dyn I<Name>Aggregate>,
}

impl Surface<Name> {
    pub fn new(aggregate: Arc<dyn I<Name>Aggregate>) -> Self {
        Self { aggregate }
    }

    pub fn handle(&self, event: &TuiEvent) -> Result<UiState, SurfaceError> {
        // orchestration only
        Ok(UiState::idle())
    }
}
