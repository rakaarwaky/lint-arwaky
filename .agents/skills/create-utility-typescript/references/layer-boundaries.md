# Layer Boundaries (Utility — TypeScript)

## Utility Layer (`utility_*.ts`)

| Allowed                                        | Forbidden                                                    |
| ---------------------------------------------- | ------------------------------------------------------------ |
| Stateless exported functions                    | Class definitions                                            |
| `this`-free functions                           | `this` keyword or class property access                      |
| Pure computation (input → output)              | Business rules or domain knowledge                           |
| I/O operations (if domain-agnostic + reusable) | Imports from Capabilities, Agent, Surface modules            |
| Taxonomy imports (`shared.taxonomy_*`)         | Implementation of protocol or aggregate interfaces           |
| Helper functions for parsing/formatting        | Magic constants (extract to `taxonomy_<domain>_constant.ts`) |
| File walking, pattern matching, validation     | Random number generation                                     |
| Environment access (if stateless + reusable)   | System clock or global state mutation                        |

## Allowed Dependencies

- `shared.taxonomy.*` — Value Objects, Constants, Entities, Events, Errors
- `shared.common.utility_*` — shared taxonomy utilities

## Forbidden Dependencies

- `capabilities_*` modules
- `agent_*` modules
- `surface_*` modules
- Concrete implementations from other layers
- Local domain data structures
