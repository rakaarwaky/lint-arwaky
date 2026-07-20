# Helper vs Utility Decision

## Keep as Private Helper (Block 3)

Keep if ANY of these is true:

1. It accesses `this.field` or instance state.
2. It accesses adapter-specific static/state.
3. It performs adapter-specific mapping.
4. It maps external errors into port-specific errors.
5. It knows infrastructure-specific configuration.
6. It is tightly coupled to this adapter only.
7. It is a factory method such as `static create()` or `static from()`.
8. It is stateless but adapter-specific and only used by this class.

## Extract to Utility (`*_utility.ts`)

Extract ONLY if ALL of these are true:

1. Stateless: no `this`, no class field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business or adapter rules.
5. Reusable: useful for multiple infrastructure/capabilities/modules.

## I/O Rule

A function with I/O can be a taxonomy utility if it is stateless, domain-agnostic, and reusable.

Stateless + I/O + domain-specific = infrastructure/port implementation.

## Decision Tree

```text
Found reusable code in infrastructure?
  │
  ├─ Does it know adapter-specific or infrastructure-specific details?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need this or class state?
  │   └─ YES → keep as helper/method in Block 3
  │
  └─ Is it stateless, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility (I/O allowed)
```
