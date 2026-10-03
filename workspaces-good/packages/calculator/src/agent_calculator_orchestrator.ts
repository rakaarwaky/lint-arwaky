/**
 * Calculator orchestrator (AES `_orchestrator`, AES101 `_aggregate`).
 *
 * The calculator member coordinates four operation features. This orchestrator
 * is the aggregate every consumer reaches: it routes a request to the feature
 * orchestrator that owns the operation, and merges their history. It performs
 * no arithmetic itself — each operation lives in its own feature folder behind
 * its own orchestrator, which the root container constructs and injects here.
 *
 * Alongside the four feature aggregates it holds the four feature operation
 * logs, so the member owns the cross-feature view: a request answered by the
 * addition feature is also written to the addition operation log, and the
 * member reads every operation log when it merges history.
 */

import { IAdditionAggregate } from "calculator-shared/src/contract_addition_aggregate";
import { ICalculatorAggregate } from "calculator-shared/src/contract_calculator_aggregate";
import { IDivisionAggregate } from "calculator-shared/src/contract_division_aggregate";
import { IMultiplicationAggregate } from "calculator-shared/src/contract_multiplication_aggregate";
import { IOperationLogProtocol } from "calculator-shared/src/contract_operation_log_protocol";
import { ISubtractionAggregate } from "calculator-shared/src/contract_subtraction_aggregate";
import { CalculatorRequest } from "calculator-shared/src/taxonomy_calculator_request";
import { CalculatorResponse } from "calculator-shared/src/taxonomy_calculator_response";
import { ExpressionVO } from "calculator-shared/src/taxonomy_expression_vo";
import { OperationVO } from "calculator-shared/src/taxonomy_operation_vo";
import { ResultVO } from "calculator-shared/src/taxonomy_result_vo";

// ─── Block 1: Struct Definition ───────────────────────────

/** The shape every operation feature aggregate presents to the member. */
interface IFeatureAggregate {
  evaluate(expr: ExpressionVO): ResultVO | null;
  history(): ResultVO[];
}

export interface CalculatorOrchestratorDeps {
  addition: IAdditionAggregate;
  subtraction: ISubtractionAggregate;
  multiplication: IMultiplicationAggregate;
  division: IDivisionAggregate;
  additionLog: IOperationLogProtocol;
  subtractionLog: IOperationLogProtocol;
  multiplicationLog: IOperationLogProtocol;
  divisionLog: IOperationLogProtocol;
}

export class CalculatorOrchestrator implements ICalculatorAggregate {
  private _features: Record<string, IFeatureAggregate>;

  constructor(deps: CalculatorOrchestratorDeps) {
    this._features = {
      [OperationVO.Add]: deps.addition,
      [OperationVO.Subtract]: deps.subtraction,
      [OperationVO.Multiply]: deps.multiplication,
      [OperationVO.Divide]: deps.division,
    };
  }

  // ─── Block 2: Aggregate Implementation ──────────────────

  execute(request: CalculatorRequest): CalculatorResponse {
    if (request.verb === "delegate") {
      return { kind: "delegation", result: this._delegate(request.expr) };
    }
    return { kind: "history", results: this._mergedHistory() };
  }

  // ─── Block 3: Helpers, Private Methods ──────────────────

  /** Route the expression to the feature that owns its operation. */
  private _delegate(expr: ExpressionVO | null): ResultVO | null {
    if (!expr) return null;
    const owner = this._features[expr.op];
    if (!owner) return null;
    return owner.evaluate(expr);
  }

  /** Merge each feature's history into one list. */
  private _mergedHistory(): ResultVO[] {
    const merged: ResultVO[] = [];
    for (const key of Object.keys(this._features)) {
      merged.push(...this._features[key].history());
    }
    return merged;
  }
}
