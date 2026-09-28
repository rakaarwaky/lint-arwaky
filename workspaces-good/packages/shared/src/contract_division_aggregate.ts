/**
 * Division aggregate contract (AES101 `_aggregate`).
 *
 * The single entry point over the division feature. A consumer passes an
 * ExpressionVO; the agent behind this aggregate hands it to the division
 * capability and returns the result, or `null` when the expression cannot be
 * divided.
 */

import type { ExpressionVO } from "./taxonomy_expression_vo";
import type { ResultVO } from "./taxonomy_result_vo";

/** Aggregate interface — the single entry point over the division feature. */
export interface IDivisionAggregate {
  /** Evaluate one division expression; `null` when the operation does not
   *  apply. */
  evaluate(expr: ExpressionVO): ResultVO | null;
}
