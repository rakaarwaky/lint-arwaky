/**
 * Subtraction feature shared mirror (AES101 `_aggregate` / AES102 `_protocol`).
 *
 * Mirror of the `packages/subtraction` feature into the `packages/shared` kernel.
 * Two FR requirements map to two protocol classes here.
 */

import type { ExpressionVO } from "../taxonomy_expression_vo";
import type { ResultVO } from "../taxonomy_result_vo";

/** Check operands capability (FR-SUBTRACTION-001). */
export interface ICheckProtocol {
  check(expr: ExpressionVO): ResultVO | null;
}

/** Run computation capability (FR-SUBTRACTION-002). */
export interface IRunProtocol {
  run(expr: ExpressionVO): ResultVO | null;
}
