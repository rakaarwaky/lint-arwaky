import { IOperationLogProtocol } from "../../shared/src/contract_operation_log_protocol";
import { OperationLogVO } from "../../shared/src/taxonomy_operation_log_vo";

// ─── Block 1: Struct Definition ─────────────────────────────

/** Merge the four operation logs into the history the caller reads. */
export class CalculatorHistoryCapability implements IOperationLogProtocol {
  // ─── Block 2: Protocol Trait Implementation ────────────────

  /** Record one evaluated result into the merged history. */
  public record(result: OperationLogVO): void {
    this.entries.push(result);
  }

  /** Return every operation log entry, newest first. */
  public merge(logs: OperationLogVO[]): OperationLogVO[] {
    const entries: OperationLogVO[] = [];
    for (const log of logs) {
      entries.push(...log.entries);
    }
    return entries.sort((a, b) => b.when - a.when);
  }

  // ─── Block 3: Constructors, Std Traits, Helpers ─────────────

  private readonly entries: OperationLogVO[] = [];

  /** A fresh capability holds no entries yet. */
  public constructor() {
    this.entries.length = 0;
  }
}
