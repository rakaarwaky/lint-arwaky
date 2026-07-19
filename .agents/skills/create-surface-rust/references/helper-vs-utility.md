# Helper vs Utility Decision

## Keep as Private Helper in Surface File

Keep if ANY of these is true:

1. It accesses `self.field` or instance state.
2. It is tightly coupled to this surface only.
3. It is a factory method such as `new()` or builder.
4. It contains surface-specific mapping logic.
5. It is stateless but only used by this one surface.

## Extract to Taxonomy Utility

Extract ONLY if ALL of these are true:

1. Stateless: no `&self`, no struct field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Reusable: useful for multiple modules/layers.
