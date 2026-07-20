# Layer Boundaries (AES)

## Agent Layer (`agent_*.py`)

| Allowed                                             | Forbidden                                             |
| --------------------------------------------------- | ----------------------------------------------------- |
| Orchestration flow (`for`, `while`, `async for`)    | Domain computation                                    |
| Control flow (`if/else`, `elif`, `match`)           | Arithmetic or analytics calculations                  |
| Sequential pipeline statements                      | Data transformation logic                             |
| Calling injected aggregate/protocol traits           | Business rules                                        |
| Error propagation (`try/except`, `raise`)           | Domain validation                                     |
| Collecting results/violations into shared VO types  | File I/O (`open()`, `Path()`, `os.`)                  |
| Async coordination (`asyncio.wait_for`)             | Network calls (`requests.`, `httpx.`)                 |
| Aggregate ABC implementation                        | Database operations (`sqlite3.`, `asyncpg.`)          |
| Private helpers supporting orchestration            | Direct stdout/stderr printing                         |
|                                                     | Direct environment/system-clock/global-state mutation |
|                                                     | Direct import from concrete `capabilities_*` modules  |
|                                                     | Direct import from concrete `utility_*` modules       |
|                                                     | Locally defined domain data structures                |
|                                                     | Raw primitives for domain values in aggregate contracts |

## Allowed Dependencies

- `shared/*`
- taxonomy VOs, constants
- aggregate ABCs
- protocol ABCs

## Forbidden Dependencies

- concrete capabilities implementations
- concrete utility implementations
