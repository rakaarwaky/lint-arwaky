/**
 * division composition root (AES root layer).
 *
 * The container wires the division evaluator and operation log behind the
 * feature's aggregate, the one place that names the concrete classes.
 */

import { IDivisionAggregate } from "calculator-shared/src/contract_division_aggregate";
import {
  DivisionOrchestrator,
  DivisionOrchestratorDeps,
} from "calculator-division/src/agent_division_orchestrator";
import { DivisionAnalyzer } from "calculator-division/src/capabilities_division_analyzer";
import { DivisionLog } from "calculator-division/src/capabilities_division_log";

// ─── Block 1: Struct Definition ───────────────────────────

export class DivisionContainer {
  private _orchestrator: IDivisionAggregate;

  constructor() {
    const log = new DivisionLog();
    this._orchestrator = new DivisionOrchestrator({
      analyzer: new DivisionAnalyzer(),
      log,
    } as DivisionOrchestratorDeps);
  }

  getOrchestrator(): IDivisionAggregate {
    return this._orchestrator;
  }
}
