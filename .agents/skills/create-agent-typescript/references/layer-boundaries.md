# Layer Boundaries (AES)

## Agent Layer (`agent_*.ts`)

| Allowed                                             | Forbidden                                             |
| --------------------------------------------------- | ----------------------------------------------------- |
| Orchestration flow (`for`, `while`, `do-while`)     | Domain computation                                    |
| Control flow (`if/else`, `switch`)                  | Arithmetic or analytics calculations                  |
| Sequential pipeline statements                      | Data transformation logic                             |
| Calling injected protocol/port traits               | Business rules                                        |
| Error propagation (`try/catch`, `throw`)            | Domain validation                                     |
| Collecting results/violations into shared VO types  | File I/O (`fs.`, `readFile`, `writeFile`)             |
| Async coordination (`Promise`, `async/await`)       | Network calls (`fetch`, `axios`, `http`)              |
| Interface implementation                            | Database operations (`sqlite3`, `pg`)                 |
| Private helpers supporting orchestration            | Direct stdout/stderr printing                         |
|                                                     | Direct environment/system-clock/global-state mutation |
|                                                     | Direct import from concrete `capabilities_*` modules  |
|                                                     | Direct import from concrete `infrastructure_*` modules |
|                                                     | Locally defined domain data structures                |
|                                                     | Raw primitives for domain values in aggregate contracts |

## Allowed Dependencies

- `shared/*`
- taxonomy VOs, constants, utilities
- protocol interfaces
- port interfaces
- aggregate interfaces

## Forbidden Dependencies

- concrete capabilities implementations
- concrete infrastructure implementations
