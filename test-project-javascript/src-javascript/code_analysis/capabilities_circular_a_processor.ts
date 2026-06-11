// PURPOSE: Test AES012 — circular import A
import { ClassB } from "./capabilities_circular_b_processor";

export class ClassA {
  b: ClassB;
  constructor() {
    this.b = new ClassB();
  }
}
