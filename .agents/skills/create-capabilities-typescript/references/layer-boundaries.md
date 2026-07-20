# Layer Boundaries (AES)

## Capabilities Layer (`capabilities_*.ts`)

| Allowed                                      | Forbidden                                             |
| -------------------------------------------- | ----------------------------------------------------- |
| Computation, validation, calculation         | Direct import from `agent_*`                          |
| Data transformation, business rules          | Direct dependency on concrete `capabilities_*` modules |
| Domain behavior using shared models          | Locally defined domain data structures                |
| Interface implementation                     |                                                       |
| External adaptation (I/O, API calls, DB)     |                                                       |
| Private helpers supporting the impl class    |                                                       |
| Calling injected port/protocol traits        |                                                       |
| Calling Utility standalone functions         |                                                       |

## Allowed Dependencies

- `shared/*`
- taxonomy VOs
- taxonomy constants
- protocol interfaces
- Utility standalone functions

## Forbidden Dependencies

- concrete agent implementations
- concrete capabilities implementations (no inter-capability deps)
