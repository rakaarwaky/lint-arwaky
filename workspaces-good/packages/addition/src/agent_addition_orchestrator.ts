/**
 * Addition orchestrator (AES `_orchestrator`, AES101 `_aggregate`).
 *
 * The addition feature owns the Add operation. This orchestrator is the entry
 * point over the feature: it coordinates the addition evaluator with the
 * addition operation log, so one call produces both the arithmetic result and
 * the durable record of it. It holds no arithmetic of its own.
 */

import { IAdditionAggregate } from "calculator-shared/src/contract_addition_aggregate";
import { ICalculatorProtocol } from "calculator-shared/src/contract_calculator_protocol";
import { IOperationLogProtocol } from "calculator-shared/src/contract_operation_log_protocol";
import { ExpressionVO } from "calculator-shared/src/taxonomy_expression_vo";
import { ResultVO } from "calculator-shared/src/taxonomy_result_vo";

// ─── Block 1: Struct Definition ───────────────────────────

export interface AdditionOrchestratorDeps {
  analyzer: ICalculatorProtocol;
  log: IOperationLogProtocol;
}

export class AdditionOrchestrator implements IAdditionAggregate {
  private _analyzer: ICalculatorProtocol;
  private _log: IOperationLogProtocol;
  private _history: ResultVO[] = [];

  constructor(deps: AdditionOrchestratorDeps) {
    this._analyzer = deps.analyzer;
    this._log = deps.log;
  }

  // ─── Block 2: Aggregate Implementation ──────────────────

  /** Evaluate one addition expression and record the result. */
  evaluate(expr: ExpressionVO): ResultVO | null {
    const result = this._analyzer.evaluate(expr);
    if (result) {
      this._history.push(result);
      this._log.record(result);
    }
    return result;
  }

  // ─── Block 3: Helpers, Private Methods ──────────────────

  /** Every result this feature has produced so far. */
  history(): ResultVO[] {
    return [...this._history];
  }
}
