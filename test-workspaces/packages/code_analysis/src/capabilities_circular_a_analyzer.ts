// AES012 — circular import test (A imports B)
import { CircularB } from "./capabilities_circular_b_checker";

export class CircularA {
  analyze(): string {
    return new CircularB().check();
  }
}
