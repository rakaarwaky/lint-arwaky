// AES013 — duplicate import test
import { CircularA } from "./capabilities_circular_a_analyzer";
import { CircularA } from "./capabilities_circular_a_analyzer";

export class DuplicateImportProcessor {
    process(): string {
        return new CircularA().analyze();
    }
}
