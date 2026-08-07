import { parseOperand } from "calculator-shared/src/utility_expression_parser";
import { CalculatorProtocol } from "calculator-shared/src/contract_calculator_protocol";
import { ExpressionVO } from "calculator-shared/src/taxonomy_expression_vo";
import {
  ResultVO,
  createResult,
} from "calculator-shared/src/taxonomy_result_vo";

export class AdditionAnalyzer implements CalculatorProtocol {
  evaluate(expr: ExpressionVO): ResultVO {
    const left = parseOperand(String(expr.left)) ?? expr.left;
    const right = parseOperand(String(expr.right)) ?? expr.right;
    const value = left + right;
    return createResult(expr.left, "+", expr.right, value);
  }
}
