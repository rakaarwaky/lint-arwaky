# Contract Roles

## Three Suffix Types

| Suffix         | Role                                               | Implemented By | Example                                   |
| -------------- | -------------------------------------------------- | -------------- | ----------------------------------------- |
| `_port`      | Outbound interface needing I/O or external systems | Infrastructure | `contract_file_system_port.py`          |
| `_protocol`  | Inbound interface for pure domain behavior         | Capabilities   | `contract_import_forbidden_protocol.py` |
| `_aggregate` | Composition facade for orchestration               | Agents         | `contract_import_runner_aggregate.py`   |

## Naming Convention

Pattern: `contract_<concept>_<role_suffix>.py`

ABC names MUST use: `I<Name>Port`, `I<Name>Protocol`, `I<Name>Aggregate`
