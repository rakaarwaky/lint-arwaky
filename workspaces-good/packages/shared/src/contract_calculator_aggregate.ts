import { ExpressionVO } from "./taxonomy_expression_vo";
import { ResultVO } from "./taxonomy_result_vo";

export interface CalculatorAggregate {
  delegate(expr: ExpressionVO): ResultVO | null;
  history(): ResultVO[];
}
