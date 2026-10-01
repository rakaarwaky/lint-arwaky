/**
 * calculator composition root (AES root layer).
 *
 * The container constructs one orchestrator per operation feature, wiring each
 * feature's evaluator and operation log behind that feature's aggregate, then
 * registers the four feature aggregates by operation and hands them to the
 * member orchestrator. This is the one place that names a concrete class.
 */

import { IAdditionAggregate } from "calculator-shared/src/contract_addition_aggregate";
import { ICalculatorAggregate } from "calculator-shared/src/contract_calculator_aggregate";
import { IDivisionAggregate } from "calculator-shared/src/contract_division_aggregate";
import { IMultiplicationAggregate } from "calculator-shared/src/contract_multiplication_aggregate";
import { ISubtractionAggregate } from "calculator-shared/src/contract_subtraction_aggregate";
import {
  AdditionOrchestrator,
  AdditionOrchestratorDeps,
} from "calculator-addition/src/agent_addition_orchestrator";
import { AdditionAnalyzer } from "calculator-addition/src/capabilities_addition_analyzer";
import { AdditionLog } from "calculator-addition/src/capabilities_addition_log";
import {
  DivisionOrchestrator,
  DivisionOrchestratorDeps,
} from "calculator-division/src/agent_division_orchestrator";
import { DivisionAnalyzer } from "calculator-division/src/capabilities_division_analyzer";
import { DivisionLog } from "calculator-division/src/capabilities_division_log";
import {
  MultiplicationOrchestrator,
  MultiplicationOrchestratorDeps,
} from "calculator-multiplication/src/agent_multiplication_orchestrator";
import { MultiplicationAnalyzer } from "calculator-multiplication/src/capabilities_multiplication_analyzer";
import { MultiplicationLog } from "calculator-multiplication/src/capabilities_multiplication_log";
import {
  SubtractionOrchestrator,
  SubtractionOrchestratorDeps,
} from "calculator-subtraction/src/agent_subtraction_orchestrator";
import { SubtractionAnalyzer } from "calculator-subtraction/src/capabilities_subtraction_analyzer";
import { SubtractionLog } from "calculator-subtraction/src/capabilities_subtraction_log";
import {
  CalculatorOrchestrator,
  CalculatorOrchestratorDeps,
} from "./agent_calculator_orchestrator";

// ─── Block 1: Struct Definition ───────────────────────────

export class CalculatorContainer {
  private _orchestrator: ICalculatorAggregate;

  constructor() {
    const additionLog = new AdditionLog();
    const subtractionLog = new SubtractionLog();
    const multiplicationLog = new MultiplicationLog();
    const divisionLog = new DivisionLog();

    const addition: IAdditionAggregate = new AdditionOrchestrator({
      analyzer: new AdditionAnalyzer(),
      log: additionLog,
    } as AdditionOrchestratorDeps);
    const subtraction: ISubtractionAggregate = new SubtractionOrchestrator({
      analyzer: new SubtractionAnalyzer(),
      log: subtractionLog,
    } as SubtractionOrchestratorDeps);
    const multiplication: IMultiplicationAggregate =
      new MultiplicationOrchestrator({
        analyzer: new MultiplicationAnalyzer(),
        log: multiplicationLog,
      } as MultiplicationOrchestratorDeps);
    const division: IDivisionAggregate = new DivisionOrchestrator({
      analyzer: new DivisionAnalyzer(),
      log: divisionLog,
    } as DivisionOrchestratorDeps);

    this._orchestrator = new CalculatorOrchestrator({
      addition,
      subtraction,
      multiplication,
      division,
      additionLog,
      subtractionLog,
      multiplicationLog,
      divisionLog,
    } as CalculatorOrchestratorDeps);
  }

  getOrchestrator(): ICalculatorAggregate {
    return this._orchestrator;
  }
}
