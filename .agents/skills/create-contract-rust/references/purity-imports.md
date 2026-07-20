# Purity and Import Restrictions (AES201)

## Allowed Imports

| Contract File               | May Import From                                                                |
| --------------------------- | ------------------------------------------------------------------------------ |
| `contract_*_protocol.rs`  | taxonomy types, other contract types, std marker traits, async_trait if needed |
| `contract_*_aggregate.rs` | taxonomy types, other contract types, std marker traits, async_trait if needed |

## Forbidden Imports

Contract files MUST NOT import from:

- `capabilities_*`
- `agent_*`
- `surface_*`
- root/container modules
- concrete implementation structs
