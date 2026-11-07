use calculator_shared::contract_calculator_protocol::IResolveProtocol;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_result_vo::ResultVO;

// ─── Block 1: Module Declaration ────────────────────────────

/// Resolves a single arithmetic expression before it runs (FR-CALCULATOR-001).
pub struct CalculatorResolveCapability;

// ─── Block 2: Module Contract ────────────────

impl IResolveProtocol for CalculatorResolveCapability {
    /// Checks the request before the run and returns the resolved value.
    fn resolve(&self, expr: &ExpressionVO) -> Option<ResultVO> {
        let _ = expr;
        None
    }

    /// Routes a request to the feature that owns the operation.
    fn route(&self, expr: &ExpressionVO) -> Option<ResultVO> {
        let _ = expr;
        None
    }
}

// ─── Block 3: Module Helpers ────────────

impl Default for CalculatorResolveCapability {
    fn default() -> Self {
        Self
    }
}
