# Contract Roles

## Two Suffix Types

| Suffix       | Role                               | Implemented By | Used By | Example                                 |
| ------------ | ---------------------------------- | -------------- | ------- | --------------------------------------- |
| `_protocol`  | Inbound interface for behavior     | Capabilities   | Agent   | `contract_import_forbidden_protocol.rs` |
| `_aggregate` | Facade for feature behavior access | Agent          | Surface | `contract_import_runner_aggregate.rs`   |

## Naming Convention

Pattern: `contract_<concept>_<role_suffix>.rs`

Trait names MUST use: `I<Name>Protocol`, `I<Name>Aggregate`
