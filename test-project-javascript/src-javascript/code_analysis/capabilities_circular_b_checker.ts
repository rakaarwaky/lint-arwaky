// AES012 — circular import test (B imports A)
import { CircularA } from "./capabilities_circular_a_analyzer";

export class CircularB {
    check(): string {
        return new CircularA().analyze();
    }
}
