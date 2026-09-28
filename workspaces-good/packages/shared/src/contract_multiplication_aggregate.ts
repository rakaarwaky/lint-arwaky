/**
 * Multiplication aggregate contract (AES101 `_aggregate`).
 *
 * The single entry point over the multiplication feature. A consumer passes an
 * ExpressionVO; the agent behind this aggregate hands it to the multiplication
 * capability and returns the result.
 */

import type { ExpressionVO } from "./taxonomy_expression_vo";
import type { ResultVO } from "./taxonomy_result_vo";

/** Aggregate interface — the single entry point over the multiplication
 *  feature. */
export interface IMultiplicationAggregate {
  /** Evaluate one multiplication expression; `null` when the operation does
   *  not apply. */
  evaluate(expr: ExpressionVO): ResultVO | null;
}
