// Fixture: AES205 — circular dependency B side (contract importing capabilities).
import { CapabilitiesProcessor } from '../capabilities/processor';

export function process() {
  const proc = new CapabilitiesProcessor();
  return proc;
}
