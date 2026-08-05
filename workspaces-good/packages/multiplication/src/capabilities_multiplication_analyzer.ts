import { CalculatorProtocol } from "calculator-shared/src/contract_calculator_protocol";
import { ExpressionVO } from "calculator-shared/src/taxonomy_expression_vo";
import {
  ResultVO,
  createResult,
} from "calculator-shared/src/taxonomy_result_vo";

export class MultiplicationAnalyzer implements CalculatorProtocol {
  evaluate(expr: ExpressionVO): ResultVO {
    const value = expr.left * expr.right;
    return createResult(expr.left, "*", expr.right, value);
  }
}
