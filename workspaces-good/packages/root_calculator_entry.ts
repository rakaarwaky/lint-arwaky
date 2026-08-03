import { ExpressionVO } from "calculator-shared/src/taxonomy_expression_vo";
import { ResultVO } from "calculator-shared/src/taxonomy_result_vo";
import { OperationVO } from "calculator-shared/src/taxonomy_operation_vo";
import { CalculatorProtocol } from "calculator-shared/src/contract_calculator_protocol";
import { AdditionAnalyzer } from "calculator-addition/src/capability_addition_analyzer";
import { SubtractionAnalyzer } from "calculator-subtraction/src/capability_subtraction_analyzer";
import { MultiplicationAnalyzer } from "calculator-multiplication/src/capability_multiplication_analyzer";
import { DivisionAnalyzer } from "calculator-division/src/capability_division_analyzer";
import { run } from "calculator-cli/src/surface_calculator_command";

export class CalculatorAggregate {
  private analyzers = new Map<OperationVO, CalculatorProtocol>();
  private _history: ResultVO[] = [];

  register(op: OperationVO, analyzer: CalculatorProtocol): void {
    this.analyzers.set(op, analyzer);
  }

  delegate(expr: ExpressionVO): ResultVO | null {
    const analyzer = this.analyzers.get(expr.op);
    if (analyzer) {
      const result = analyzer.evaluate(expr);
      if (result) this._history.push(result);
      return result;
    }
    return null;
  }

  history(): ResultVO[] {
    return this._history;
  }
}

// --- CLI entry point ---
const calc = new CalculatorAggregate();
calc.register(OperationVO.Add, new AdditionAnalyzer());
calc.register(OperationVO.Subtract, new SubtractionAnalyzer());
calc.register(OperationVO.Multiply, new MultiplicationAnalyzer());
calc.register(OperationVO.Divide, new DivisionAnalyzer());

const args = process.argv.slice(2);
const command = args[0] || "run";

if (command === "run") {
  run(calc);
} else {
  console.error(`Unknown command: ${command}`);
  console.error("Usage: main.ts [run]");
  process.exit(1);
}
