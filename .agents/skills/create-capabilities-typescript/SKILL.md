---
name: create-capabilities-typescript
description: "Create and validate TypeScript capabilities layer files following AES rules: concrete implementation of behavior (business logic + external adaptation), 3-block structure, one class per file, protocol interface contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
  tags:
    [
      typescript,
      aes,
      capability,
      protocol,
      structure,
      3-block-structure,
      di,
      vo,
      role-naming,
    ]
  triggers:
    - "create capability typescript"
    - "add capability typescript"
    - "fix capability structure typescript"
    - "create protocol typescript"
    - "capability missing protocol typescript"
    - "check capabilities typescript"
    - "audit capabilities typescript"
  dependencies: []
  related:
    - create-agent-typescript
    - create-contract-typescript
    - create-taxonomy-typescript
---

# create-capabilities-typescript

## Purpose

Create and validate TypeScript **capabilities layer** files following AES rules.

A capabilities file contains the **concrete implementation** of the system's behavior. This layer encapsulates both:

- **Business logic**: computations, validations, transformations, assessments
- **External adaptation**: database access, third-party API calls, file system access, infrastructure mechanics

Capabilities hide these implementations behind Contracts, keeping behavior modular, swappable, and fully isolated from orchestration.

A capabilities file must:

- implement one domain protocol interface (via `implements` keyword),
- follow strict 3-block structure,
- use dependency injection for service collaborators,
- use shared VOs for domain data,
- use Utility standalone functions for low-level technical operations.

## Role Naming (ARCHITECTURE §8)

Capabilities use role suffixes describing their concern. Two families:

**Internal (business logic):**

validator, assessor, calculator, resolver, classifier, selector, mapper, transformer, policy, enricher, evaluator, analyzer, scorer, grader, ranker, filter, checker, reviewer, approver, rejector

**External (adaptation):**

repository, gateway, client, provider, fetcher, reader, writer, scanner, executor, publisher, subscriber, adapter, connector, uploader, downloader, sender, receiver, dispatcher, watcher, monitor

File: `capabilities_<domain>_<role>.ts`

## Dependencies (ARCHITECTURE §8)

- **May depend on:** Taxonomy, Contract, Utility.
- **Must NOT depend on / import:** other Capabilities, Agent.

Note: do **not** import `infrastructure_*` — that layer no longer exists; its mechanics now live in the Utility layer. Utility is an allowed dependency.

## Special Rules (ARCHITECTURE §8)

- **No Inter-Capability Dependency:** a capability never imports or calls another capability. They are standalone execution units.
- **Pipeline Aggregation:** multiple capabilities are composed into a sequential pipeline by the **Agent layer**, not by themselves.
- **Shared Logic Extraction (DRY):** if several capabilities need the same technical mechanics, extract it into a reusable standalone function in the **Utility layer**. Capabilities must not duplicate technical code.
- **Contract Implementation:** the capability implements the protocol interface defined in the Contract layer. The file MUST import from `_protocol` module only (not `_port`). Example: `import { I<Name> } from '..._protocol';`
- **State Ownership:** the capability owns business and technical state within its execution scope.
- **Utility Delegation:** low-level technical operations call Utility standalone functions, passing state/data as arguments.
- **No Orchestration:** no flow control across capabilities (looping/branching between capabilities) and no error-escalation policy. Execute one responsibility, return a result.
- **No Domain Definition:** do not define domain models (Entities, Value Objects); only consume and produce Taxonomy.
- **Constant Extraction:** extract reusable constants (magic strings, numbers, patterns) into `taxonomy_<domain>_constant.ts` in shared. Capabilities must not contain magic constants.

## AES403 — Capability Composition Rules

The `CapabilitiesRoleChecker` enforces 3 rules per file:

### Rule 1: Internal Helpers Are Allowed

Classes without `implements` are **allowed** and never flagged. These are internal helper types that don't implement protocol behavior. They must not start with `_`.

```typescript
// ✅ ALLOWED — internal helper, no implements keyword needed
class Cache {
  private data: Array<string>;
}
```

### Rule 2: At Least One Implementor Required

The file MUST have at least one class that implements a protocol interface via `implements`. If no class implements an interface → flag `CapabilityNoImplementor` (AES403, Severity MEDIUM).

```typescript
// ✅ PASSING — implements IAgentRoleChecker
class MyChecker implements IAgentRoleChecker {
  checkContainer(source: string, violations: any[]) {}
}

// ❌ FAILING — no implements keyword
class HelperA {
  doA() {}
}

class HelperB {
  doB() {}
}
```

### Rule 3: Maximum 3 Types Per File

Total type declarations (class, interface, enum) must not exceed **3**. If exceeded → flag `CapabilityTooManyTypes` (AES403, Severity HIGH).

**Important:** `type` aliases are NOT counted — they are not new types.

```typescript
// ✅ PASSING — exactly 3 types (1 class + 1 interface + 1 enum)
class Cap implements IAgentRoleChecker {
  checkContainer(source: string, violations: any[]) {}
}

interface Helper {}

enum Status { Active, Inactive }

// ❌ FAILING — 4 types exceeds limit
class Cap implements IAgentRoleChecker {
  checkContainer(source: string, violations: any[]) {}
}

class A {}
class B {}
enum C { X, Y }
```

### Detection Patterns

| Language | Implementor Pattern | Non-Implementor |
|----------|-------------------|-----------------|
| Rust | `impl Trait for Struct` | Standalone `impl Struct` |
| Python | `class Name(Protocol):` | Standalone `class Name:` |
| TS | `class Name implements IProto` | Standalone `class Name {}` |

The guard check requires `_protocol` import only (no `_port` allowed):

```typescript
// ✅ Guard passes — imports from _protocol module
import { IAgentRoleChecker } from 'shared/role_rules/contract_agent_role_protocol';

// ❌ Guard fails — no _protocol import → CapabilityNoProtocol
export class MyChecker {
  doWork() {}
}
```

## Definition of Done

1. ONE implementation class per file.
2. Class implements ONE domain protocol interface in Block 2.
3. Block 2 contains ONLY the domain protocol method implementations.
4. Utility methods, static factories, private helpers in Block 3.
5. No locally defined domain models — Entities/Value Objects are consumed from Taxonomy, not defined here.
6. Service dependencies use DI via protocol interfaces.
7. Value/configuration fields use shared VOs.
8. No inter-capability dependencies (capabilities must not import other capabilities or Agent).
9. Low-level technical operations delegate to Utility standalone functions.
10. Reusable constants extracted to `taxonomy_<domain>_constant.ts` in shared.
11. At least one class implements a protocol interface (Rule 2).
12. Total type count ≤ 3 (class + interface + enum, not counting `type` aliases) (Rule 3).
13. File imports from `_protocol` module only (no `_port`).
14. `npx tsc --noEmit` passes.

## References

Read these files for detailed rules:

| File                                | Content                                                |
| ----------------------------------- | ------------------------------------------------------ |
| `references/layer-boundaries.md`    | Allowed/Forbidden imports and dependencies             |
| `references/3-block-structure.md`   | Block 1/2/3 definitions, method placement rules        |
| `references/helper-vs-utility.md`   | Helper vs utility decision, I/O Blocker, decision tree |
| `references/primitive-vo-policy.md` | Primitive policy table, VO construction rules          |
| `references/error-handling.md`      | Error handling rules with examples                     |
| `references/examples.md`            | All BAD/GOOD code examples                             |
| `references/commands.md`            | Quick heuristic check commands                         |
| `references/checklist.md`           | Verification checklist                                 |

## Templates

Use these templates when creating new files:

| File                                  | Purpose                              |
| ------------------------------------- | ------------------------------------ |
| `templates/capabilities_name.ts`      | New capabilities implementation file |
| `templates/contract_name_protocol.ts` | New protocol interface definition    |

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask: **"Does this implement protocol behavior?"**

If yes → keep as capabilities. If no → check if it's orchestration (→ agent), domain data (→ taxonomy), or pure technical mechanics (→ utility).

### Step 2: Check Protocol Import (AES403 Guard)

The file MUST import from a `_protocol` module. If missing → flag `CapabilityNoProtocol`.

```typescript
import { I<Name> } from '..._protocol';
```

### Step 3: Create Interface File if Missing

Create `contract_<name>_protocol.ts` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: class definition + constructor → protocol methods → utility/factories/helpers.

### Step 5: Verify AES403 Compliance

- **Rule 1:** Internal helper classes without `implements` are ALLOWED (never flagged).
- **Rule 2:** At least one class must implement a protocol interface (`class Name implements IProto`).
- **Rule 3:** Total type count ≤ 3 (class + interface + enum, not counting `type` aliases).

### Step 6: Verify Class Discipline

One class, no local data interfaces, DI via protocol interfaces, shared VOs.

### Step 7: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 8: Verify Layer Compliance

No forbidden imports (Agent, other capabilities), no inter-capability dependencies, no business logic leakage, no domain model definition.

### Step 9: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 10: Verify Compilation

```bash
npx tsc --noEmit
```

## Quick Commands

```bash
# Check forbidden imports (no agent, no other capabilities)
grep -n "^\s*from\s+.*agent_\|^\s*from\s+.*capabilities_" packages/*/src/capabilities_*.ts

# List protocol interface implementations
grep -n "implements I[A-Za-z0-9_]*Protocol" packages/*/src/capabilities_*.ts

# Check _protocol import (guard)
grep -n "import.*_protocol" packages/*/src/capabilities_*.ts
```

## Common Mistakes

- Importing other capabilities or Agent directly.
- Defining domain models (Entities, Value Objects) in capabilities files.
- Using concrete service types as constructor fields.
- Putting private helpers in the protocol interface.
- Putting constructors in the protocol interface.
- Placing utility methods before the domain protocol methods.
- Mixing Block 2 and Block 3 responsibilities.
- Flow control across capabilities / error-escalation policy (orchestration).
- Silent error swallowing with `?? ''` or `|| 0`.
- Magic constants in capabilities logic (extract to `taxonomy_<domain>_constant.ts`).
- Not delegating low-level technical operations to Utility.
- Importing from `_port` module instead of `_protocol` module.
- Having no class that implements a protocol interface (Rule 2 violation).
- Exceeding 3 total types in a file (Rule 3 violation).
