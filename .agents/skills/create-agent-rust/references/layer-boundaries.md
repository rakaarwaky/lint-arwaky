# Layer Boundaries (AES)

## Agent Layer (`agent_*.rs`)

| Allowed                                             | Forbidden                                             |
| --------------------------------------------------- | ----------------------------------------------------- |
| Orchestration flow (`for`, `while`, `loop`)         | Domain computation                                    |
| Control flow (`if/else`, `match`)                   | Arithmetic or analytics calculations                  |
| Sequential pipeline statements                      | Data transformation logic                             |
| Calling injected protocol/port traits               | Business rules                                        |
| Error propagation (`?`, `match`, `if let`)          | Domain validation                                     |
| Collecting results/violations into shared VO types  | File I/O (`std::fs`, `File::open`)                    |
| Async coordination (`select!`, `join!`)             | Network calls (`reqwest`, `hyper`)                    |
| Aggregate trait implementation                      | Database operations (`sqlx`, `rusqlite`)              |
| Private helpers supporting orchestration            | Direct stdout/stderr printing                         |
|                                                     | Direct environment/system-clock/global-state mutation |
|                                                     | Direct import from concrete `capabilities_*` modules  |
|                                                     | Direct import from concrete `infrastructure_*` modules |
|                                                     | Direct import from concrete `surface_*` modules       |
|                                                     | Locally defined domain data structures                |
|                                                     | Raw primitives for domain values in aggregate contracts |

## Allowed Dependencies

- `shared/*`
- taxonomy VOs, constants, utilities
- protocol traits
- port traits
- aggregate traits

## Forbidden Dependencies

- concrete capabilities implementations
- concrete infrastructure implementations
- concrete surface implementations
