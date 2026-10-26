/**
 * Calculator feature shared mirror (AES101 `_aggregate` / AES102 `_protocol`).
 *
 * Mirror of the `packages/calculator` feature into the `packages/shared` kernel.
 * Two FR requirements map to two protocol classes here.
 */

import type { ExpressionVO } from "../taxonomy_expression_vo";
import type { ResultVO } from "../taxonomy_result_vo";

/** Route request capability (FR-CALCULATOR-001). */
export interface IRouteProtocol {
  route(expr: ExpressionVO): ResultVO | null;
}

/** Merge history capability (FR-CALCULATOR-002). */
export interface IMergeProtocol {
  merge(expr: ExpressionVO): ResultVO | null;
}
