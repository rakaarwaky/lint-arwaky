# Helper vs Utility Decision

## Keep as Private Helper (Block 3)

Keep if ANY of these is true:

1. It accesses `self.field` or instance state.
2. It is tightly coupled to this orchestrator only.
3. It is a factory method such as `create_default()` or `from_config()`.
4. It contains agent-specific pipeline knowledge.
5. It contains domain knowledge.
6. It is stateless but only used by this one class.

## Extract to Utility (`*_utility.py`)

Extract ONLY if ALL of these are true:

1. Stateless: no `self`, no `cls`, no class field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business or orchestration rules.
5. Reusable: useful for multiple modules/layers.

## I/O Rule

A function with I/O can be a taxonomy utility if it is stateless, domain-agnostic, and reusable.

Stateless + I/O + domain-specific = capabilities implementation.

## Decision Tree

```text
Found reusable code in agent?
  │
  ├─ Does it know agent-specific or domain-specific details?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need self or class state?
  │   └─ YES → keep as helper/method in Block 3
  │
  └─ Is it stateless, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility (I/O allowed)
```
