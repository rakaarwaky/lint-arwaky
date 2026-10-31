/**
 * Calculator history merge capability (AES `_capability`, `IMergeProtocol`).
 *
 * The calculator member owns the cross-operation view of every feature log, so
 * one capability merges the logs into a single newest-first list that the
 * orchestrator returns for history reads.
 */

import { IMergeProtocol } from "calculator-shared/src/calculator/contract_calculator_protocol";
import { ResultVO } from "../../shared/src/taxonomy_result_vo";

// ─── Block 1: Struct Definition ─────────────────────────────

/** Merge the operation results into the history the caller reads. */
export class CalculatorHistoryCapability implements IMergeProtocol {
  // ─── Block 2: Protocol Trait Implementation ────────────────

  /** Record one evaluated result into the merged history. */
  public record(result: ResultVO): void {
    this.entries.push(result);
  }

  /** Return every recorded result, newest first. */
  public merge(logs: ResultVO[][]): ResultVO[] {
    const entries: ResultVO[] = [];
    for (const log of logs) {
      entries.push(...log);
    }
    return entries.reverse();
  }

  // ─── Block 3: Constructors, Std Traits, Helpers ───────────

  private readonly entries: ResultVO[] = [];

  /** A fresh capability holds no entries yet. */
  public constructor() {
    this.entries.length = 0;
  }
}
