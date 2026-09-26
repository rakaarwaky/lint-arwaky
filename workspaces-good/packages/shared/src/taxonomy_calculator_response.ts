/**
 * CalculatorResponse — response VOs for the calculator aggregate.
 */

import { ResultVO } from "./taxonomy_result_vo";

/** Results of a calculator aggregate request. */
export type CalculatorResponse =
  | { kind: "delegation"; result: ResultVO | null }
  | { kind: "history"; results: ResultVO[] };
