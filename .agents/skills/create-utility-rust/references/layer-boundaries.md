# Layer Boundaries (Utility)

## Utility Layer (`utility_*.rs`)

| Allowed                                        | Forbidden                                                    |
| ---------------------------------------------- | ------------------------------------------------------------ |
| Stateless free functions                       | Struct definitions                                           |
| Pure computation (input → output)              | `&self` or struct field access                               |
| I/O operations (if domain-agnostic + reusable) | Business rules or domain knowledge                           |
| Taxonomy imports (`shared::taxonomy_*`)        | Imports from Capabilities, Agent, Surface modules            |
| Contract imports (for type resolution only)    | Implementation of protocol or aggregate traits               |
| Helper functions for parsing/formatting        | Magic constants (extract to `taxonomy_<domain>_constant.rs`) |
| File walking, pattern matching, validation     | Random number generation                                     |
| Environment access (if stateless + reusable)   | System clock or global state mutation                        |

## Allowed Dependencies

- `shared/taxonomy/*` — Value Objects, Constants, Entities, Events, Errors
- `shared/common/utility_*` — shared taxonomy utilities

## Forbidden Dependencies

- `capabilities_*` modules
- `agent_*` modules
- `surface_*` modules
- Concrete implementations from other layers
- Local domain data structures
