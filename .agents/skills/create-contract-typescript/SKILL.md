---
name: create-contract-typescript
description: "Create and validate TypeScript contract layer files in shared domain: pure interface definitions for protocols and aggregates. Contracts define public promises only, with no implementation, no layer imports, and domain-safe VO-based signatures."
metadata:
    tags: [typescript, aes, contract, protocol, aggregate, interface, shared, di, vo]
    triggers:
        - "create contract typescript"
        - "add contract typescript"
        - "create protocol typescript"
        - "create aggregate typescript"
        - "fix contract typescript"
        - "check contract typescript"
        - "audit contract typescript"
    dependencies: []
    related:
        - create-taxonomy-typescript
        - create-capabilities-typescript
        - create-agent-typescript
---

# create-contract-typescript

## Purpose

Create and validate TypeScript **contract layer** files in shared domain.

Contracts are pure interface definitions.

They define the **WHAT**: public promises, stable interfaces, polymorphism boundaries, DI boundaries.

They MUST NOT define the **HOW**: no implementation, no private helpers, no I/O, no business logic, no layer imports.

Two contract suffixes serve different roles:

- `_protocol` → implemented by Capabilities (inbound behavior interface)
- `_aggregate` → implemented by Agent (facade for Surface to access feature behavior)

## Definition of Done

1. Contract file uses correct suffix: `_protocol` or `_aggregate`.
2. Contract contains only interface definitions.
3. Contract contains no method implementations or private helper signatures.
4. Interface is exported with `export interface`.
5. Methods have proper TypeScript type annotations.
6. Contract imports only taxonomy and contract types.
7. Contract signatures use shared VOs for domain data.
8. New contract module is registered in `index.ts`.
9. `npx tsc --noEmit` passes.

## References

| File | Content |
|------|---------|
| `references/contract-roles.md` | Two suffix types and naming convention |
| `references/purity-imports.md` | AES201 import restrictions |
| `references/interface-structure-rules.md` | 7 interface structure rules |
| `references/primitive-vo-policy.md` | Primitive policy table |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | Verification checklist |

## Templates

| File | Purpose |
|------|---------|
| `templates/contract_name_protocol.ts` | New protocol interface definition |
| `templates/contract_name_aggregate.ts` | New aggregate interface definition |

## Workflow

### Step 1: Determine the Contract Role

Ask: **"Which layer will implement this interface?"**

| Implemented By | Suffix |
|----------------|--------|
| Capabilities | `_protocol` |
| Agent | `_aggregate` |

### Step 2: Identify Public Methods

Apply the Golden Rule: Is this method called by outer layers? YES → keep in contract. NO → make it a private helper.

### Step 3: Create Contract File

Create `contract_<concept>_<suffix>.ts` in the appropriate shared domain folder.

### Step 4: Register Module

Update the domain `index.ts`.

### Step 5: Verify

```bash
npx tsc --noEmit
```

## Quick Commands

```bash
# List contract interfaces
grep -n "^export interface I[A-Za-z0-9_]*Protocol\|^export interface I[A-Za-z0-9_]*Aggregate" packages/shared/src/**/contract_*.ts

# Check forbidden imports
grep -n "from.*capabilities_\|from.*agent_\|from.*surface_" packages/shared/src/*/contract_*.ts
```

## Common Mistakes

- Putting implementation logic in contract files.
- Adding method implementations to contract interfaces.
- Importing concrete layer types into contracts.
- Using wrong suffix for contract files.
- Leaking implementation details into contract interfaces.
- Missing type annotations on methods.
- Using raw `string` for domain values in contract signatures.
- Forgetting to register contract modules in `index.ts`.
- Forgetting to export interfaces with `export interface`.
