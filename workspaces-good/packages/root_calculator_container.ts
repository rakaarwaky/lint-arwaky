import { CalculatorAggregate } from "calculator-shared/src/contract_calculator_aggregate";
import {
  CalculatorOrchestrator,
  CalculatorOrchestratorDeps,
} from "./agent_calculator_orchestrator";
import { AdditionAnalyzer } from "calculator-addition/src/capabilities_addition_analyzer";
import { SubtractionAnalyzer } from "calculator-subtraction/src/capabilities_subtraction_analyzer";
import { MultiplicationAnalyzer } from "calculator-multiplication/src/capabilities_multiplication_analyzer";
import { DivisionAnalyzer } from "calculator-division/src/capabilities_division_analyzer";

// ─── Block 1: Struct Definition ───────────────────────────

export class CalculatorContainer {
  private orchestrator: CalculatorOrchestrator;

  constructor() {
    this.orchestrator = new CalculatorOrchestrator({
      addition: new AdditionAnalyzer(),
      subtraction: new SubtractionAnalyzer(),
      multiplication: new MultiplicationAnalyzer(),
      division: new DivisionAnalyzer(),
    } as CalculatorOrchestratorDeps);
  }

  getOrchestrator(): CalculatorAggregate {
    return this.orchestrator;
  }
}
