# Layer Boundaries (AES)

## Capabilities Layer (`capabilities_*.ts`)

| Allowed                                   | Forbidden                                              |
| ----------------------------------------- | ------------------------------------------------------ |
| Computation, validation, calculation      | Direct import from.*capabilities_|from.*agent_|from.*surface_*`                           |
| Data transformation, business rules       | Direct dependency on concrete `capabilities_*` modules |
| Domain behavior using shared models       | Locally defined domain data structures                 |
| Interface implementation                  |                                                        |
| External adaptation (I/O, API calls, DB)  |                                                        |
| Private helpers supporting the impl class |                                                        |
| Calling injected protocol traits          |                                                        |
| Calling Utility standalone functions      |                                                        |

## Allowed Dependencies (ARCHITECTURE §8)

A capability may depend ONLY on these layers:

- **Taxonomy** — VOs, constants, entities, events
- **Contract** — protocol interfaces, aggregate interfaces
- **Utility** — standalone stateless functions (the utility layer handles these concerns)

## Forbidden Dependencies (ARCHITECTURE §8)

- concrete **agent** implementations — capabilities must not import or know about the Agent layer
- concrete **capabilities** implementations — no inter-capability dependencies; capabilities never import each other

## Special Rules (ARCHITECTURE §8)

- **No Inter-Capability Dependency:** a capability never imports or calls another capability. They are standalone execution units.
- **Pipeline Aggregation:** multiple capabilities are composed into a sequential pipeline by the **Agent layer**, not by themselves.
- **Shared Logic Extraction (DRY):** if several capabilities need the same technical mechanics, extract it into a reusable standalone function in the **Utility layer**. Capabilities must not duplicate technical code.
- **Contract Implementation:** the capability implements the `protocol_` interface defined in the Contract layer.
- **State Ownership:** the capability owns business and technical state within its execution scope.
- **Utility Delegation:** low-level technical operations call Utility standalone functions, passing state/data as arguments.
- **No Orchestration:** no flow control across capabilities (looping/branching between capabilities) and no error-escalation policy. Execute one responsibility, return a result.
- **No Domain Definition:** do not define domain models (Entities, Value Objects); only consume and produce Taxonomy.
