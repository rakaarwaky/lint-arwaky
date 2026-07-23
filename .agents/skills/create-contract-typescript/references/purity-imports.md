# Purity and Import Restrictions (AES201)

## Allowed Imports

| Contract File             | May Import From                      |
| ------------------------- | ------------------------------------ |
| `contract_*_protocol.ts`  | taxonomy types, other contract types |
| `contract_*_aggregate.ts` | taxonomy types, other contract types |

## Forbidden Imports

Contract files MUST NOT import from:

- `capabilities_*`
- `agent_*`
- `surface_*`
- concrete implementation classes
