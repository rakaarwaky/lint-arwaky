/**
 * subtraction composition root (AES root layer).
 *
 * The container wires the subtraction evaluator and operation log behind the
 * feature's aggregate, the one place that names the concrete classes.
 */

import { ISubtractionAggregate } from "calculator-shared/src/contract_subtraction_aggregate";
import {
  SubtractionOrchestrator,
  SubtractionOrchestratorDeps,
} from "calculator-subtraction/src/agent_subtraction_orchestrator";
import { SubtractionAnalyzer } from "calculator-subtraction/src/capabilities_subtraction_analyzer";
import { SubtractionLog } from "calculator-subtraction/src/capabilities_subtraction_log";

// ─── Block 1: Struct Definition ───────────────────────────

export class SubtractionContainer {
  private _orchestrator: ISubtractionAggregate;

  constructor() {
    const log = new SubtractionLog();
    this._orchestrator = new SubtractionOrchestrator({
      analyzer: new SubtractionAnalyzer(),
      log,
    } as SubtractionOrchestratorDeps);
  }

  getOrchestrator(): ISubtractionAggregate {
    return this._orchestrator;
  }
}
