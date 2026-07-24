# Examples

## BAD: Computation in Agent

```typescript
class <Name>Orchestrator {
    process(files: FilePath[]) {
        const total = files.length; // BAD: computation
        const sum = files.reduce((acc, f) => acc + f.size, 0); // BAD
    }
}
```

## BAD: Business Logic in Agent

```typescript
class <Name>Orchestrator {
    evaluate(content: FileContent): boolean {
        return content.value.includes("forbidden-marker"); // BAD: business rule
    }
}
```

## BAD: I/O in Agent

```typescript
class <Name>Orchestrator {
    execute(path: FilePath) {
        const content = fs.readFileSync(path.value()); // BAD
    }
}
```

## BAD: Interface in Agent File

```typescript
interface <Name>ReportVO {
    results: string[];
}
```

## BAD: Concrete Service Field

```typescript
class <Name>Orchestrator {
    constructor(private readonly service: <ServiceType>) {} // BAD: concrete type
}
```

## GOOD: Correct 3-Block Order

```typescript
import { I<Protocol>Protocol } from '../shared/<domain>/contract_<protocol>_protocol';
import { I<Name>Aggregate } from '../shared/<domain>/contract_<name>_aggregate';
import { <RequestVO> } from '../shared/<domain>/taxonomy_<request>_vo';
import { <ResultVO> } from '../shared/<domain>/taxonomy_<result>_vo';

export class <Name>Orchestrator implements I<Name>Aggregate {
    constructor(private readonly deps: <Name>Deps) {}

    execute(request: <RequestVO>): <ResultVO> {
        const formatter: I<Protocol>Protocol = this.getFormatter(request);
        return formatter.process(request);
    }

    toString(): string {
        return '<Name>Orchestrator()';
    }

    private getFormatter(request: <RequestVO>): I<Protocol>Protocol {
        switch (request.type) {
            case RequestType.A: return this.deps.a;
            case RequestType.B: return this.deps.b;
        }
    }

    static create(): <Name>Orchestrator {
        return new <Name>Orchestrator({ a: new Default<Protocol>(), b: new Default<Protocol>() });
    }
}
```
