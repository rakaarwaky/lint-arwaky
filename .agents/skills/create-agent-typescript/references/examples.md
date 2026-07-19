# Examples

## BAD: Computation in Agent

```typescript
class OrphanOrchestrator {
    process(files: FilePath[]) {
        const total = files.length; // BAD: computation
        const sum = files.reduce((acc, f) => acc + f.size, 0); // BAD
    }
}
```

## BAD: Business Logic in Agent

```typescript
class OrphanOrchestrator {
    analyze(content: FileContent): boolean {
        return content.value.includes("orphan"); // BAD: business rule
    }
}
```

## BAD: I/O in Agent

```typescript
class OrphanOrchestrator {
    execute(path: FilePath) {
        const content = fs.readFileSync(path.value()); // BAD
    }
}
```

## BAD: Interface in Agent File

```typescript
interface OrphanReport {
    results: string[];
}
```

## BAD: Concrete Service Field

```typescript
class OrphanOrchestrator {
    constructor(private readonly analyzer: OrphanAnalyzer) {} // BAD
}
```

## GOOD: Correct 3-Block Order

```typescript
import { IOrphanAnalyzerProtocol } from '../shared/orphan_detector/contract_orphan_protocol';
import { IOrphanOrchestratorAggregate } from '../shared/orphan_detector/contract_orphan_aggregate';
import { LintResult } from '../shared/code_analysis/taxonomy_result_vo';

export class OrphanOrchestrator implements IOrphanOrchestratorAggregate {
    constructor(private readonly analyzer: IOrphanAnalyzerProtocol) {}

    execute(request: ScanRequest): LintResult[] {
        const violations: LintResult[] = [];
        for (const file of request.files()) {
            try {
                const result = this.analyzer.analyze(file);
                violations.push(...result.intoViolations());
            } catch (err) {
                violations.push(LintResult.fromAnalysisError(file, err));
            }
        }
        return violations;
    }

    toString(): string {
        return 'OrphanOrchestrator()';
    }

    static create(): OrphanOrchestrator {
        return new OrphanOrchestrator(new CapabilitiesOrphanAnalyzer());
    }
}
```
