/**
 * CalculatorRequest / CalculatorResponse — aggregate request/response VOs for
 * the calculator domain. The aggregate's single `execute()` entry point takes
 * a CalculatorRequest and returns the matching CalculatorResponse; each
 * variant carries VO-wrapped values, not raw primitives.
 */

import { ExpressionVO } from "./taxonomy_expression_vo";
import { ResultVO } from "./taxonomy_result_vo";

/** Consumer verbs carried by the calculator aggregate's single entry point. */
export type CalculatorRequest =
  { verb: "delegate"; expr: ExpressionVO } | { verb: "history" };

/** Results of a calculator aggregate request. */
export type CalculatorResponse =
  | { kind: "delegation"; result: ResultVO | null }
  | { kind: "history"; results: ResultVO[] };

export function requestDelegate(expr: ExpressionVO): CalculatorRequest {
  return { verb: "delegate", expr };
}

export function requestHistory(): CalculatorRequest {
  return { verb: "history" };
}
