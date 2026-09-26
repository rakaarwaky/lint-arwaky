/**
 * calculator-domain capability contracts (AES102 `_protocol`).
 *
 * One file for the calculator feature. Each interface below is one capability
 * seam: an interface carries every method that capability implements, with one
 * concrete return type each, so a capability implements its interface outright
 * and never carries stubs.
 */

import type { ExpressionVO } from "./taxonomy_expression_vo";
import type { ResultVO } from "./taxonomy_result_vo";

/** Arithmetic evaluation capability: evaluates one expression using this
 *  capability's own operation. */
export interface ICalculatorProtocol {
  /** Evaluate a single arithmetic expression. Return the result, or `null`
   *  when the operation does not apply (e.g. division by zero). */
  evaluate(expr: ExpressionVO): ResultVO | null;
}
