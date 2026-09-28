import { IOperationLogProtocol } from "calculator-shared/src/contract_operation_log_protocol";
import { OperationVO } from "calculator-shared/src/taxonomy_operation_vo";
import { ResultVO } from "calculator-shared/src/taxonomy_result_vo";

/** Records the results the addition feature produced, and only those. */
export class AdditionLog implements IOperationLogProtocol {
  private _entries: ResultVO[] = [];

  record(result: ResultVO): void {
    if (result.expression.indexOf(OperationVO.Add) === -1) return;
    this._entries.push(result);
  }

  /** Every addition result recorded so far, oldest first. */
  entries(): ResultVO[] {
    return [...this._entries];
  }
}
