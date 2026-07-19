# Purity and Import Restrictions (AES201)

Taxonomy must remain pure and stable.

## Allowed Dependencies

| Taxonomy Type | May Import From                              | Must Not Import From                                                |
| ------------- | -------------------------------------------- | ------------------------------------------------------------------- |
| `_vo`       | other taxonomy types, stdlib                | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_entity`   | other taxonomy types, stdlib                | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_error`    | other taxonomy types, stdlib                | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_event`    | other taxonomy types, stdlib                | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_constant` | only core/static values                      | external layer imports, I/O, functions                              |
| `_utility`  | taxonomy types, pure stdlib helpers         | capabilities, infrastructure, agents, surface, root, contracts, I/O |

## Taxonomy May Contain

- value validation,
- domain invariants inside constructors,
- pure transformations between taxonomy types.

## Taxonomy Must Not Contain

- file I/O, network calls, database access, environment mutation,
- side effects, business orchestration, use-case logic, layer-specific behavior.
