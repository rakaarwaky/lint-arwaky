# Purity and Import Restrictions (AES201)

## Allowed Imports

| Contract File               | May Import From                          |
| --------------------------- | ---------------------------------------- |
| `contract_*_port.py`      | taxonomy types, other contract types     |
| `contract_*_protocol.py`  | taxonomy types, other contract types     |
| `contract_*_aggregate.py` | taxonomy types, other contract types     |

## Forbidden Imports

Contract files MUST NOT import from:

- `capabilities_*`
- `infrastructure_*`
- `agent_*`
- `surface_*`
- concrete implementation classes
