# Layer Boundaries (AES)

## Agent Layer (`agent_*.ts`)

| Allowed                                             | Forbidden                                             |
| --------------------------------------------------- | ----------------------------------------------------- |
| Orchestration flow (`for`, `while`, `async for`)    | Domain computation                                    |
| Control flow (`if/else`, `elif`, `match`)           | Arithmetic or analytics calculations                  |
| Sequential pipeline statements                      | Data transformation logic                             |
| Calling injected aggregate/protocol interfaces       | Business rules                                        |
| Error propagation (`try/catch`, `throw`)            | Domain validation                                     |
| Collecting results/violations into shared VO types  | File I/O (`fs.`, `readFile`, `writeFile`)             |
| Async coordination (`Promise.all`, `race`)           | Network calls (`fetch`, `axios`, `http`)              |
| Aggregate interface implementation                  | Database operations (`sqlite3`, `pg`)                 |
| Private helpers supporting orchestration            | Direct stdout/stderr printing                         |
|                                                     | Direct environment/system-clock/global-state mutation |
|                                                     | Direct import from concrete `capabilities_*` modules  |
|                                                     | Direct import from concrete `utility_*` modules       |
|                                                     | Locally defined domain data structures                |
|                                                     | Raw primitives for domain values in aggregate contracts |

## Allowed Dependencies

- `shared/*`
- taxonomy VOs, constants
- aggregate interfaces
- protocol interfaces

## Forbidden Dependencies

- concrete capabilities implementations
- concrete utility implementations
