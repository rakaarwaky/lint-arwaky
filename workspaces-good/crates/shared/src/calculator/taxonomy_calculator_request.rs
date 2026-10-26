// PURPOSE: CalculatorRequest — request VOs for the calculator aggregate.

use crate::taxonomy_expression_vo::ExpressionVO;

/// Consumer verbs carried by the calculator aggregate's single entry point.
pub enum CalculatorRequest {
    /// Resolve a single arithmetic expression and return the result.
    Resolve { expr: ExpressionVO },
    /// Run a single arithmetic expression and return the result.
    Run { expr: ExpressionVO },
}
