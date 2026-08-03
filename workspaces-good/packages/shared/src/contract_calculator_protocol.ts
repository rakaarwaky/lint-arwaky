import { ExpressionVO } from "./taxonomy_expression_vo";
import { ResultVO } from "./taxonomy_result_vo";

export interface CalculatorProtocol {
  evaluate(expr: ExpressionVO): ResultVO | null;
}
