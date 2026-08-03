import { CalculatorAggregate } from "calculator-shared/src/contract_calculator_aggregate";
import { CalculatorProtocol } from "calculator-shared/src/contract_calculator_protocol";
import { ExpressionVO } from "calculator-shared/src/taxonomy_expression_vo";
import { OperationVO } from "calculator-shared/src/taxonomy_operation_vo";
import { ResultVO } from "calculator-shared/src/taxonomy_result_vo";
import { AdditionAnalyzer } from "calculator-addition/src/capability_addition_analyzer";
import { SubtractionAnalyzer } from "calculator-subtraction/src/capability_subtraction_analyzer";
import { MultiplicationAnalyzer } from "calculator-multiplication/src/capability_multiplication_analyzer";
import { DivisionAnalyzer } from "calculator-division/src/capability_division_analyzer";
import { run } from "calculator-cli/src/surface_calculator_command";

class Calculator implements CalculatorAggregate {
  private hist: ResultVO[] = [];
  private analyzers: Record<OperationVO, CalculatorProtocol>;

  constructor() {
    this.analyzers = {
      [OperationVO.Add]: new AdditionAnalyzer(),
      [OperationVO.Subtract]: new SubtractionAnalyzer(),
      [OperationVO.Multiply]: new MultiplicationAnalyzer(),
      [OperationVO.Divide]: new DivisionAnalyzer(),
    };
  }

  delegate(expr: ExpressionVO): ResultVO | null {
    const analyzer = this.analyzers[expr.op];
    const result = analyzer ? analyzer.evaluate(expr) : null;
    if (result) this.hist.push(result);
    return result;
  }

  history(): ResultVO[] {
    return [...this.hist];
  }
}

const calc = new Calculator();
run(calc);
