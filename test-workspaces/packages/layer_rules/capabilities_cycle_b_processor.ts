// AES205: circular dependency
import { AProtocol } from "./contract_cycle_a_protocol";
class BProcessor {
  getB(): string {
    return "B";
  }
}
