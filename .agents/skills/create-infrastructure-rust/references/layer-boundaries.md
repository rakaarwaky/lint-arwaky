# Layer Boundaries (AES)

## Infrastructure Layer (`infrastructure_*.rs`)

| Allowed                                             | Forbidden                                             |
| --------------------------------------------------- | ----------------------------------------------------- |
| File I/O (`std::fs`, `File::open`, `read_dir`)      | Business rules                                        |
| Network calls (`reqwest`, `hyper`)                  | Domain logic                                          |
| Database operations (`sqlx`, `rusqlite`)            | Domain calculations                                   |
| External API calls                                  | Domain validation that decides business correctness   |
| Environment/system access via controlled adapter    | Direct import from concrete `agent_*` modules         |
| Serialization/deserialization                       | Direct import from concrete `capabilities_*` modules  |
| Technical mapping (DTO ↔ VO)                        | Locally defined domain data structures                |
| Error mapping from external libraries               | Raw primitives for domain values in public contracts  |
| Port trait implementation                           | Silent error swallowing                               |
| Private helpers supporting the adapter              |                                                       |

## Allowed Dependencies

- `shared/*`
- taxonomy VOs, constants, utilities
- port traits
- protocol traits defined in shared, when required by the adapter contract

## Forbidden Dependencies

- concrete capabilities implementations
- concrete agent implementations
