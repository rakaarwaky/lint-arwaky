# Examples

## BAD: Capability Without Interface (AES403)

```typescript
export class FrameComposer {
    composeFrame(): void {
        // public behavior without protocol interface
    }
}
```

Fix:

```typescript
export class FrameComposer implements IFrameComposerProtocol {
    composeFrame(): void {
        // contract implementation
    }
}
```

## BAD: I/O in Capabilities (AES404)

```typescript
export class MyCapability {
    process(): void {
        const content = fs.readFileSync('file.txt', 'utf-8'); // FORBIDDEN
    }
}
```

## BAD: Interface Defined in Layer File

```typescript
interface OrphanResult {
    isOrphan: boolean;
    reason: string;
}
```

Fix: Move to shared taxonomy, then import.

## BAD: Concrete Service Field

```typescript
export class CapabilitiesOrphanAnalyzer {
    constructor(private readonly extractor: FilenameExtractor) {} // BAD
}
```

Fix:

```typescript
export class CapabilitiesOrphanAnalyzer {
    constructor(private readonly extractor: IOrphanFilenameExtractorProtocol) {}
}
```

## BAD: Orchestration Inside Capability (No Orchestration, §8)

```typescript
export class MyPipeline implements IImportCheckerProtocol {
    run(): void {
        const a = this.stepA();      // calls another capability's behavior
        if (a.isOk()) {
            this.stepB();            // branching between capabilities
        } else {
            this.escalate();         // error-escalation policy
        }
    }
}
```

Fix: remove flow control and cross-capability calls. Let the Agent layer compose the pipeline. The capability executes one responsibility and returns a result.

## BAD: Domain Model Defined in Capability (No Domain Definition, §8)

```typescript
interface OrphanResult {   // domain model defined here = forbidden
    isOrphan: boolean;
    reason: string;
}
```

Fix: define `OrphanResult` as a Taxonomy VO; the capability only consumes and produces it.

## BAD: Utility Methods in Block 2

```typescript
export class ArchLineChecker implements ILineCheckerProtocol {
    constructor() {}

    toString(): string {                    // ← Block 2 position, NOT a protocol method
        return 'ArchLineChecker()';
    }

    checkLineCounts(...): void {            // ← pushed down
    }
}
```

Fix: Move `toString()` to Block 3.

## GOOD: Capability with DI and Shared VO

```typescript
import { OrphanAnalysisPolicy } from '../shared/orphan_detector/taxonomy_orphan_analysis_policy_vo';
import { IOrphanFileCachePort } from '../shared/orphan_detector/contract_orphan_file_cache_port';
import { IOrphanFilenameExtractorProtocol } from '../shared/orphan_detector/contract_orphan_filename_extractor_protocol';
import { ICapabilitiesOrphanProtocol } from '../shared/orphan_detector/contract_capabilities_orphan_protocol';

export class CapabilitiesOrphanAnalyzer implements ICapabilitiesOrphanProtocol {
    constructor(
        private readonly extractor: IOrphanFilenameExtractorProtocol,
        private readonly cache: IOrphanFileCachePort,
        private readonly policy: OrphanAnalysisPolicy,
    ) {}
}
```

## GOOD: Correct 3-Block Structure

```typescript
import { FilePath } from '../shared/code_analysis/taxonomy_file_path_vo';
import { LayerDefinition } from '../shared/code_analysis/taxonomy_layer_definition_vo';
import { ILineCheckerProtocol } from '../shared/code_analysis/contract_line_checker_protocol';
import { isBarrelFile } from '../shared/code_analysis/taxonomy_line_checker_utility';
import { LintResult } from '../shared/code_analysis/taxonomy_lint_result_vo';
import { SourceContentVO } from '../shared/code_analysis/taxonomy_source_vo';

// ─── Block 1: Class Definition & Constructor ──────────────
export class ArchLineChecker implements ILineCheckerProtocol {
    constructor() {}

    // ─── Block 2: Public Contract (domain protocol ONLY) ──
    checkLineCounts(
        file: FilePath,
        definition: LayerDefinition | null,
        source: SourceContentVO,
        violations: LintResult[],
    ): void {
        const basename = file.basename();
        if (isBarrelFile(basename)) {
            return;
        }
        // Remaining domain logic...
    }

    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return 'ArchLineChecker()';
    }

    equals(other: unknown): boolean {
        return other instanceof ArchLineChecker;
    }

    static create(): ArchLineChecker {
        return new ArchLineChecker();
    }
}
```
