// Re-export the aggregate contract declared in the calculator subdir so that
// `calculator_shared::contract_calculator_aggregate::ICalculatorAggregate`
// resolves while `fn execute` lives under `crates/shared/src/calculator/`
// for the AES601 protocol/aggregate method check.
pub use crate::calculator::contract_calculator_aggregate::ICalculatorAggregate;

// AES202: contract(aggregate) layer must import taxonomy. Re-export the
// taxonomy types used by the aggregate trait so they are part of this
// module's public interface and satisfy the mandatory taxonomy import.
pub use crate::taxonomy_calculator_request::CalculatorRequest;
pub use crate::taxonomy_calculator_response::CalculatorResponse;
