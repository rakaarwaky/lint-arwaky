/** Time one merge of the four operation logs. */
import { performance } from "perf_hooks";

/** Return the milliseconds one merge takes over a representative log set. */
export function mergeBenchmark(): number {
  const start = performance.now();
  for (let i = 0; i < 1000; i += 1) {
    merge([]);
  }
  return performance.now() - start;
}
