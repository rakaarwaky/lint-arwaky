/**
 * CalculatorRequest — request VOs for the calculator aggregate.
 */

import { ExpressionVO } from "./taxonomy_expression_vo";

/** Consumer verbs carried by the calculator aggregate's single entry point. */
export type CalculatorRequest =
  { verb: "delegate"; expr: ExpressionVO } | { verb: "history" };

export function requestDelegate(expr: ExpressionVO): CalculatorRequest {
  return { verb: "delegate", expr };
}

export function requestHistory(): CalculatorRequest {
  return { verb: "history" };
}
