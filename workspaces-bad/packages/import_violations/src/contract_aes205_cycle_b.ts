// Fixture: AES205 — circular dependency B side (contract imports capabilities).
import { process as capProcess } from './capabilities_aes205_cycle_a';

export class ContractCycle {
  run() {
    return capProcess();
  }
}
