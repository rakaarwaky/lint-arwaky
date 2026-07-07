// AES205: circular dependency
import { BProcessor } from "./capabilities_cycle_b_processor";
class AProtocol {
  getA(): string {
    return "A";
  }
}
