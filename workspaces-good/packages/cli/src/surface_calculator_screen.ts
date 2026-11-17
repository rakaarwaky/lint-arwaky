// Surface fixture: unified surface self-import — `surface` is allowed to import
// other `surface` files (row 10 of the 11-scope AES201 Import Matrix).
import { run as commandRun } from "./surface_calculator_command";
import { ICalculatorAggregate } from "calculator-shared/src/contract_calculator_aggregate";

export function show(calc: ICalculatorAggregate): void {
  commandRun(calc);
}
