/**
 * calculator-domain aggregate contract (AES101 `_aggregate`).
 *
 * The single entry point over the calculator feature. Consumers pass a
 * CalculatorRequest; the agent behind the aggregate dispatches to the rich
 * protocol interface in `contract_calculator_protocol.ts`.
 */

import type {
  CalculatorRequest,
  CalculatorResponse,
} from "./taxonomy_calculator_request_vo";

/** Aggregate interface — the single entry point over the calculator feature. */
export interface ICalculatorAggregate {
  /** Execute a calculator request and return the corresponding response. */
  execute(request: CalculatorRequest): CalculatorResponse;
}
