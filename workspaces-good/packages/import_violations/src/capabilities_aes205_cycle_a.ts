// Fixture: AES205 — circular dependency A side.
import { ContractAggregate } from '../contract/aggregate';

export function process() {
  const agg = new ContractAggregate();
  return agg;
}
