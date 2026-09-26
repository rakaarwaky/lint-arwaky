import { ICalculatorAggregate } from "calculator-shared/src/contract_calculator_aggregate";
import { ICalculatorProtocol } from "calculator-shared/src/contract_calculator_protocol";
import {
  CalculatorRequest,
  CalculatorResponse,
} from "calculator-shared/src/taxonomy_calculator_request_vo";
import { ExpressionVO } from "calculator-shared/src/taxonomy_expression_vo";
import { OperationVO } from "calculator-shared/src/taxonomy_operation_vo";
import { ResultVO } from "calculator-shared/src/taxonomy_result_vo";

// ─── Block 1: Struct Definition ───────────────────────────

export interface CalculatorOrchestratorDeps {
  addition: ICalculatorProtocol;
  subtraction: ICalculatorProtocol;
  multiplication: ICalculatorProtocol;
  division: ICalculatorProtocol;
}

export class CalculatorOrchestrator implements ICalculatorAggregate {
  private deps: CalculatorOrchestratorDeps;
  private _history: ResultVO[] = [];

  constructor(deps: CalculatorOrchestratorDeps) {
    this.deps = deps;
  }

  // ─── Block 2: Aggregate Implementation ──────────────────

  execute(request: CalculatorRequest): CalculatorResponse {
    if (request.verb === "delegate") {
      return { kind: "delegation", result: this._delegate(request.expr) };
    }
    return { kind: "history", results: [...this._history] };
  }

  // ─── Block 3: Helpers, Private Methods ──────────────────

  private _delegate(expr: ExpressionVO): ResultVO | null {
    const analyzerMap: Partial<Record<OperationVO, ICalculatorProtocol>> = {
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
}
