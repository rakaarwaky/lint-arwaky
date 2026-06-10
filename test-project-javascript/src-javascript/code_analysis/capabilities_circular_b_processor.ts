// PURPOSE: Test AES012 — circular import B
import { ClassA } from "./capabilities_circular_a_processor";

export class ClassB {
    a: ClassA;
    constructor() {
        this.a = new ClassA();
    }
}
