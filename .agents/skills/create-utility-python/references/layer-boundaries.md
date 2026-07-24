# Layer Boundaries (Utility — Python)

## Utility Layer (`utility_*.py`)

| Allowed                                        | Forbidden                                                    |
| ---------------------------------------------- | ------------------------------------------------------------ |
| Stateless module-level functions                | Class definitions                                            |
| `self`-less functions                           | `self` parameter or class attribute access                   |
| Pure computation (input → output)              | Business rules or domain knowledge                           |
| I/O operations (if domain-agnostic + reusable) | Imports from Capabilities, Agent, Surface modules            |
| Taxonomy imports (`shared.taxonomy_*`)         | Implementation of protocol or aggregate ABCs                 |
| Helper functions for parsing/formatting        | Magic constants (extract to `taxonomy_<domain>_constant.py`) |
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
