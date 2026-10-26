// PURPOSE: calculator-domain aggregate contract (AES101 `_aggregate`).

use crate::taxonomy_calculator_request::CalculatorRequest;
use crate::taxonomy_calculator_response::CalculatorResponse;

/// Aggregate trait — the single entry point over the calculator feature.
pub trait ICalculatorAggregate: Send + Sync {
    /// Execute a calculator request and return the corresponding response.
    fn execute(&self, request: CalculatorRequest) -> CalculatorResponse;
}
