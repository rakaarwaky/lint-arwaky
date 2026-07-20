# Layer Boundaries (AES)

## Capabilities Layer (`capabilities_*.rs`)

| Allowed                                      | Forbidden                                             |
| -------------------------------------------- | ----------------------------------------------------- |
| Computation, validation, calculation         | Direct import from `agent_*`                          |
| Data transformation, business rules          | Direct dependency on concrete `capabilities_*` modules |
| Domain behavior using shared models          | Locally defined domain data structures                |
| Protocol trait implementation                |                                                       |
| External adaptation (I/O, API calls, DB)     |                                                       |
| Private helpers supporting the impl struct   |                                                       |
| Calling injected port/protocol traits        |                                                       |
| Calling Utility standalone functions         |                                                       |

## Allowed Dependencies

- `shared/*`
- taxonomy VOs
- taxonomy constants
- protocol traits
- Utility standalone functions

## Forbidden Dependencies

- concrete agent implementations
- concrete capabilities implementations (no inter-capability deps)
