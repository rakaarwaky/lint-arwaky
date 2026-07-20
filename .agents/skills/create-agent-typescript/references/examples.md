# Examples

## BAD: Computation in Agent

```typescript
class <NameOrchestrator> {
    process(files: FilePath[]) {
        const total = files.length; // BAD: computation
        const sum = files.reduce((acc, f) => acc + f.size, 0); // BAD
    }
}
```

## BAD: Business Logic in Agent

```typescript
class <NameOrchestrator> {
    evaluate(content: FileContent): boolean {
        return content.value.includes("forbidden-marker"); // BAD: business rule
    }
}
```

## BAD: I/O in Agent

```typescript
class <NameOrchestrator> {
    execute(path: FilePath) {
        const content = fs.readFileSync(path.value()); // BAD
    }
}
```

## BAD: Interface in Agent File

```typescript
interface <Report>VO {
    results: string[];
}
```

## BAD: Concrete Service Field

```typescript
class <NameOrchestrator> {
    constructor(private readonly analyzer: <NameAnalyzer>) {} // BAD
}
```

## GOOD: Correct 3-Block Order

```typescript
import { I<NameAnalyzer>Protocol } from '../shared/<name-feature>/contract_analyzer_protocol';
import { I<NameOrchestrator>Aggregate } from '../shared/<name-feature>/contract_orchestrator_aggregate';
import { <ResultVO> } from '../shared/<name-feature>/taxonomy_result_vo';

export class <NameOrchestrator> implements I<NameOrchestrator>Aggregate {
    constructor(private readonly analyzer: I<NameAnalyzer>Protocol) {}

    execute(request: <ScanRequest>VO): <ResultVO>[] {
        const results: <ResultVO>[] = [];
        for (const file of request.files()) {
            try {
                const result = this.analyzer.analyze(file);
                results.push(...result.intoResults());
            } catch (err) {
                results.push(<ResultVO>.fromAnalysisError(file, err));
            }
        }
        return results;
    }

    toString(): string {
        return '<NameOrchestrator>()';
    }

    static create(): <NameOrchestrator> {
        return new <NameOrchestrator>(new Capabilities<NameCapability>());
    }
}
```
