import { IOperationLogProtocol } from "calculator-shared/src/contract_operation_log_protocol";
import { OperationVO } from "calculator-shared/src/taxonomy_operation_vo";
import { ResultVO } from "calculator-shared/src/taxonomy_result_vo";

// ─── Block 1: Struct Definition ────────────────────────────

/** Records the results the multiplication feature produced, and only those. */
export class MultiplicationLog implements IOperationLogProtocol {
  private _entries: ResultVO[] = [];

  // ─── Block 2: Protocol Trait Implementation ────────────────

  record(result: ResultVO): void {
    if (!this._isMultiplicationResult(result)) return;
    this._entries.push(result);
  }

  /** Every multiplication result recorded so far, oldest first. */
  entries(): ResultVO[] {
    return [...this._entries];
  }

  // ─── Block 3: Constructors, Std Traits, Helpers ────────────

  private _isMultiplicationResult(result: ResultVO): boolean {
    return result.expression.indexOf(OperationVO.Multiply) !== -1;
  }
}
