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
export class <NameCapability> {
    process(): void {
        const content = fs.readFileSync('file.txt', 'utf-8'); // FORBIDDEN
    }
}
```

## BAD: Interface Defined in Layer File

```typescript
interface <NameResult> {
    is_valid: boolean;
    reason: string;
}
```

Fix: Move to shared taxonomy, then import.

## BAD: Concrete Service Field

```typescript
export class Capabilities<NameCapability> {
    constructor(private readonly collaborator: <NameCollaborator>) {} // BAD
}
```

Fix:

```typescript
export class Capabilities<NameCapability> {
    constructor(private readonly collaborator: I<NameCollaborator>Protocol) {}
}
```

## BAD: Orchestration Inside Capability (No Orchestration, §8)

```typescript
export class <NamePipeline> implements I<NameCapability>Protocol {
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
interface <NameResult> {   // domain model defined here = forbidden
    is_valid: boolean;
    reason: string;
}
```

Fix: define `<NameResult>` as a Taxonomy VO; the capability only consumes and produces it.

## BAD: Utility Methods in Block 2

```typescript
export class Capabilities<NameCapability> implements I<NameCapability>Protocol {
    constructor() {}

    toString(): string {                    // ← Block 2 position, NOT a protocol method
        return 'Capabilities<NameCapability>()';
    }

    execute(...): void {            // ← pushed down
    }
}
```

Fix: Move `toString()` to Block 3.

## GOOD: Capability with DI and Shared VO

```typescript
import { <NamePolicy>VO } from '../shared/<name-feature>/taxonomy_<name-policy>_vo';
import { I<NameStore>Protocol } from '../shared/<name-feature>/contract_<name-store>_protocol';
import { I<NameCollaborator>Protocol } from '../shared/<name-feature>/contract_<name-collaborator>_protocol';
import { I<NameCapability>Protocol } from '../shared/<name-feature>/contract_<name-capability>_protocol';

export class Capabilities<NameCapability> implements I<NameCapability>Protocol {
    constructor(
        private readonly collaborator: I<NameCollaborator>Protocol,
        private readonly store: I<NameStore>Protocol,
        private readonly policy: <NamePolicy>VO,
    ) {}
}
```

## GOOD: Correct 3-Block Structure

```typescript
import { <DomainVO> } from '../shared/<name-feature>/taxonomy_<domain>_vo';
import { I<NameCapability>Protocol } from '../shared/<name-feature>/contract_<name-capability>_protocol';
import { <name>_utility } from '../shared/<name-feature>/taxonomy_<name-utility>';
import { <ResultVO> } from '../shared/<name-feature>/taxonomy_<result>_vo';

// ─── Block 1: Class Definition & Constructor ──────────────
export class Capabilities<NameCapability> implements I<NameCapability>Protocol {
    constructor() {}

    // ─── Block 2: Public Contract (domain protocol ONLY) ──
    execute(
        input: <DomainVO>,
        output: <ResultVO>[],
    ): void {
        const key = input.key();
        if (<name>_utility(key)) {
            return;
        }
        // Remaining domain logic...
    }

    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return 'Capabilities<NameCapability>()';
    }

    equals(other: unknown): boolean {
        return other instanceof Capabilities<NameCapability>;
    }

    static create(): Capabilities<NameCapability> {
        return new Capabilities<NameCapability>();
    }
}
```
