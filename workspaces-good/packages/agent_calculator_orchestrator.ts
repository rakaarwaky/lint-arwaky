import { CalculatorAggregate } from "calculator-shared/src/contract_calculator_aggregate";
import { CalculatorProtocol } from "calculator-shared/src/contract_calculator_protocol";
import { ExpressionVO } from "calculator-shared/src/taxonomy_expression_vo";
import { OperationVO } from "calculator-shared/src/taxonomy_operation_vo";
import { ResultVO } from "calculator-shared/src/taxonomy_result_vo";

// ─── Block 1: Struct Definition ───────────────────────────

export interface CalculatorOrchestratorDeps {
  addition: CalculatorProtocol;
  subtraction: CalculatorProtocol;
  multiplication: CalculatorProtocol;
  division: CalculatorProtocol;
}

// ─── Block 2: Protocol Implementation ─────────────────────

export class CalculatorOrchestrator implements CalculatorAggregate {
  private deps: CalculatorOrchestratorDeps;
  private _history: ResultVO[] = [];

  constructor(deps: CalculatorOrchestratorDeps) {
    this.deps = deps;
  }

  delegate(expr: ExpressionVO): ResultVO | null {
    const analyzerMap: Partial<Record<OperationVO, CalculatorProtocol>> = {
      [OperationVO.Add]: this.deps.addition,
      [OperationVO.Subtract]: this.deps.subtraction,
      [OperationVO.Multiply]: this.deps.multiplication,
      [OperationVO.Divide]: this.deps.division,
    };
    const analyzer = analyzerMap[expr.op];
    if (!analyzer) return null;
    const result = analyzer.evaluate(expr);
    if (result) this._history.push(result);
    return result;
  }

  history(): ResultVO[] {
    return [...this._history];
  }
}
