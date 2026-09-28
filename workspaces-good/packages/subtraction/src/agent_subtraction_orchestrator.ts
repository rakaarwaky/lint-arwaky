/**
 * Subtraction orchestrator (AES `_orchestrator`, AES101 `_aggregate`).
 *
 * The subtraction feature owns the Subtract operation. This orchestrator is
 * the entry point over the feature: it coordinates the subtraction evaluator
 * with the subtraction operation log, so one call produces both the
 * arithmetic result and the durable record of it. It holds no arithmetic of
 * its own.
 */

import { ICalculatorProtocol } from "calculator-shared/src/contract_calculator_protocol";
import { IOperationLogProtocol } from "calculator-shared/src/contract_operation_log_protocol";
import { ISubtractionAggregate } from "calculator-shared/src/contract_subtraction_aggregate";
import { ExpressionVO } from "calculator-shared/src/taxonomy_expression_vo";
import { ResultVO } from "calculator-shared/src/taxonomy_result_vo";

// ─── Block 1: Struct Definition ───────────────────────────

export interface SubtractionOrchestratorDeps {
  analyzer: ICalculatorProtocol;
  log: IOperationLogProtocol;
}

export class SubtractionOrchestrator implements ISubtractionAggregate {
  private _analyzer: ICalculatorProtocol;
  private _log: IOperationLogProtocol;
  private _history: ResultVO[] = [];

  constructor(deps: SubtractionOrchestratorDeps) {
    this._analyzer = deps.analyzer;
    this._log = deps.log;
  }

  // ─── Block 2: Aggregate Implementation ──────────────────

  /** Evaluate one subtraction expression and record the result. */
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
