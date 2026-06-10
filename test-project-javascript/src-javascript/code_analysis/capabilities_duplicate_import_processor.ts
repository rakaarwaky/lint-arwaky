// PURPOSE: Test AES013 — duplicate import
import { ClassA } from "./capabilities_circular_a_processor";
import { ClassA } from "./capabilities_circular_a_processor";

export class DuplicateImportClass {
    process(): number {
        return 42;
    }
}
