// PURPOSE: calculator-domain aggregate contract (AES101 `_aggregate`).
//
// The single entry point over the calculator feature. Consumers pass a
// CalculatorRequest; the agent behind the aggregate dispatches to the rich
// protocol trait in `contract_calculator_protocol.rs`.

use crate::taxonomy_calculator_request_vo::{CalculatorRequest, CalculatorResponse};

/// Aggregate trait — the single entry point over the calculator feature.
pub trait ICalculatorAggregate: Send + Sync {
    /// Execute a calculator request and return the corresponding response.
    fn execute(&self, request: CalculatorRequest) -> CalculatorResponse;
}
