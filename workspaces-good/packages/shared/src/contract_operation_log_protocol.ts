/**
 * Operation log capability contract (AES102 `_protocol`).
 *
 * One file for the operation-log seam. Each operation feature wires a recorder
 * next to its evaluator, so a feature folder holds two capability seams and
 * its orchestrator coordinates both: the arithmetic result and the durable
 * record of that result.
 */

import { ResultVO } from "./taxonomy_result_vo";

export interface IOperationLogProtocol {
  /** Record one evaluated result into the operation log. */
  record(result: ResultVO): void;
}
