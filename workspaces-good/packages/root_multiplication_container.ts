/**
 * multiplication composition root (AES root layer).
 *
 * The container wires the multiplication evaluator and operation log behind the
 * feature's aggregate, the one place that names the concrete classes.
 */

import { IMultiplicationAggregate } from "calculator-shared/src/contract_multiplication_aggregate";
import {
  MultiplicationOrchestrator,
  MultiplicationOrchestratorDeps,
} from "calculator-multiplication/src/agent_multiplication_orchestrator";
import { MultiplicationAnalyzer } from "calculator-multiplication/src/capabilities_multiplication_analyzer";
import { MultiplicationLog } from "calculator-multiplication/src/capabilities_multiplication_log";

// ─── Block 1: Struct Definition ───────────────────────────

export class MultiplicationContainer {
  private _orchestrator: IMultiplicationAggregate;

  constructor() {
    const log = new MultiplicationLog();
    this._orchestrator = new MultiplicationOrchestrator({
      analyzer: new MultiplicationAnalyzer(),
      log,
    } as MultiplicationOrchestratorDeps);
  }

  getOrchestrator(): IMultiplicationAggregate {
    return this._orchestrator;
  }
}
