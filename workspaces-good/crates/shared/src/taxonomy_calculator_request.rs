// PURPOSE: CalculatorRequest — request VOs for the calculator aggregate.

use crate::taxonomy_expression_vo::ExpressionVO;

/// Consumer verbs carried by the calculator aggregate's single entry point.
pub enum CalculatorRequest {
    /// Evaluate a single arithmetic expression and return the result.
    Delegate { expr: ExpressionVO },
    /// Return the full calculation history so far.
    History,
}

impl CalculatorRequest {
    pub fn delegate(expr: ExpressionVO) -> Self {
        Self::Delegate { expr }
    }

    pub fn history() -> Self {
        Self::History
    }
}
