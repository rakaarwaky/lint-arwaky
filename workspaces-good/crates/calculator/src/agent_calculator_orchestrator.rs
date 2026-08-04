use calculator_shared::contract_calculator_aggregate::CalculatorAggregate;
use calculator_shared::contract_calculator_protocol::CalculatorProtocol;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_operation_vo::OperationVO;
use calculator_shared::taxonomy_result_vo::ResultVO;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CalculatorOrchestratorDeps {
    pub addition: Box<dyn CalculatorProtocol>,
    pub subtraction: Box<dyn CalculatorProtocol>,
    pub multiplication: Box<dyn CalculatorProtocol>,
    pub division: Box<dyn CalculatorProtocol>,
}

pub struct CalculatorOrchestrator {
    deps: CalculatorOrchestratorDeps,
    history: Vec<ResultVO>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl CalculatorAggregate for CalculatorOrchestrator {
    fn delegate(&mut self, expr: &ExpressionVO) -> Option<ResultVO> {
        let result = match expr.op {
            OperationVO::Add => self.deps.addition.evaluate(expr),
            OperationVO::Subtract => self.deps.subtraction.evaluate(expr),
            OperationVO::Multiply => self.deps.multiplication.evaluate(expr),
            OperationVO::Divide => self.deps.division.evaluate(expr),
        };
        if let Some(ref r) = result {
            self.history.push(r.clone());
        }
        result
    }

    fn history(&self) -> Vec<ResultVO> {
        self.history.clone()
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl CalculatorOrchestrator {
    pub fn new(deps: CalculatorOrchestratorDeps) -> Self {
        Self {
            deps,
            history: Vec::new(),
        }
    }
}
