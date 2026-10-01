import { IOperationLogProtocol } from "calculator-shared/src/contract_operation_log_protocol";
import { OperationVO } from "calculator-shared/src/taxonomy_operation_vo";
import { ResultVO } from "calculator-shared/src/taxonomy_result_vo";

// ─── Block 1: Struct Definition ────────────────────────────

/** Records the results the subtraction feature produced, and only those. */
export class SubtractionLog implements IOperationLogProtocol {
  private _entries: ResultVO[] = [];

  // ─── Block 2: Protocol Trait Implementation ────────────────

  record(result: ResultVO): void {
    if (!this._isSubtractionResult(result)) return;
    this._entries.push(result);
  }

  /** Every subtraction result recorded so far, oldest first. */
  entries(): ResultVO[] {
    return [...this._entries];
  }

  // ─── Block 3: Constructors, Std Traits, Helpers ────────────

  private _isSubtractionResult(result: ResultVO): boolean {
    return result.expression.indexOf(OperationVO.Subtract) !== -1;
  }
}
