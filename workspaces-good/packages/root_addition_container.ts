/**
 * addition composition root (AES root layer).
 *
 * The container wires the addition evaluator and operation log behind the
 * feature's aggregate, the one place that names the concrete classes.
 */

import { IAdditionAggregate } from "calculator-shared/src/contract_addition_aggregate";
import {
  AdditionOrchestrator,
  AdditionOrchestratorDeps,
} from "calculator-addition/src/agent_addition_orchestrator";
import { AdditionAnalyzer } from "calculator-addition/src/capabilities_addition_analyzer";
import { AdditionLog } from "calculator-addition/src/capabilities_addition_log";

// ─── Block 1: Struct Definition ───────────────────────────

export class AdditionContainer {
  private _orchestrator: IAdditionAggregate;

  constructor() {
    const log = new AdditionLog();
    this._orchestrator = new AdditionOrchestrator({
      analyzer: new AdditionAnalyzer(),
      log,
    } as AdditionOrchestratorDeps);
  }

  getOrchestrator(): IAdditionAggregate {
    return this._orchestrator;
  }
}
