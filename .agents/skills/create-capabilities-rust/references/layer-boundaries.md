# Layer Boundaries (AES)

## Capabilities Layer (`capabilities_*.rs`)

| Allowed                                      | Forbidden                                             |
| -------------------------------------------- | ----------------------------------------------------- |
| Computation, validation, calculation         | File I/O (`std::fs`, `File::open`, `read_dir`)        |
| Data transformation, business rules          | Network calls (`reqwest`, `hyper`)                    |
| Domain behavior using shared models          | Database operations (`sqlx`, `rusqlite`)              |
| Protocol trait implementation                | Direct stdout/stderr printing                         |
| Private helpers supporting the impl struct   | Direct environment/system-clock/global-state mutation |
| Calling injected port/protocol traits        | Direct import from `infrastructure_*`                 |
|                                              | Direct import from `agent_*`                          |
|                                              | Direct dependency on concrete `capabilities_*` modules |
|                                              | Locally defined domain data structures                |

## Allowed Dependencies

- `shared/*`
- taxonomy VOs
- taxonomy constants
- taxonomy utilities
- protocol/port traits

## Forbidden Dependencies

- concrete infrastructure implementations
- concrete agent implementations
