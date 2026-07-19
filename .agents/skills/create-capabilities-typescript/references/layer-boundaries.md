# Layer Boundaries (AES)

## Capabilities Layer (`capabilities_*.ts`)

| Allowed                                      | Forbidden                                             |
| -------------------------------------------- | ----------------------------------------------------- |
| Computation, validation, calculation         | File I/O (`fs.`, `readFile`, `writeFile`)             |
| Data transformation, business rules          | Network calls (`fetch`, `axios`, `http`)              |
| Domain behavior using shared models          | Database operations (`sqlite3`, `pg`)                 |
| Interface implementation                     | Direct stdout/stderr printing                         |
| Private helpers supporting the impl class    | Direct environment/system-clock/global-state mutation |
| Calling injected port/protocol traits        | Direct import from `infrastructure_*`                 |
|                                              | Direct import from `agent_*`                          |
|                                              | Direct dependency on concrete `capabilities_*` modules |
|                                              | Locally defined domain data structures                |

## Allowed Dependencies

- `shared/*`
- taxonomy VOs
- taxonomy constants
- taxonomy utilities
- protocol/port interfaces

## Forbidden Dependencies

- concrete infrastructure implementations
- concrete agent implementations
