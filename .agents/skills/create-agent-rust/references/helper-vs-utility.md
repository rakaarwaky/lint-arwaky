# Helper vs Utility Decision

## Keep as Private Helper (Block 3)

Keep if ANY of these is true:

1. It accesses `self.field` or instance state.
2. It is tightly coupled to this orchestrator only.
3. It is a factory method such as `new()` or builder.
4. It contains agent-specific pipeline knowledge.
5. It contains domain knowledge.
6. It is stateless but only used by this one struct.

## Extract to Utility (`*_utility.rs`)

Extract ONLY if ALL of these are true:

1. Stateless: no `&self`, no struct field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business or orchestration rules.
5. Reusable: useful for multiple modules/layers.

## I/O Blocker

Stateless + I/O = infrastructure/port implementation, NOT taxonomy utility, NOT agent layer.

## Decision Tree

```text
Found reusable code in agent?
  │
  ├─ Does it know agent-specific or domain-specific details?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need &self or struct state?
  │   └─ YES → keep as helper/method in Block 3
  │
  ├─ Does it perform I/O or side effects?
  │   └─ YES → move to infrastructure/port implementation
  │
  └─ Is it stateless, pure, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility
```
