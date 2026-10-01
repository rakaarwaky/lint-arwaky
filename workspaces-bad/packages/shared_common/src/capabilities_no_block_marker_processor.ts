// AES403: no block markers at all.
// A capability declares Block 1 (class) -> Block 2 (protocol methods) ->
// Block 3 (utility methods, factories, helpers). Each block needs its banner so
// the reader is given a map of the file. This one declares none, so
// `check_block_markers` reports "declares no block markers".
import { SomeProtocol } from "shared/src/contract_some_protocol";

export class NoBlockMarkerProcessor implements SomeProtocol {
  private _seen: number = 0;

  execute(value: string): boolean {
    return !value;
  }
}
