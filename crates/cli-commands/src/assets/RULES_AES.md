# AES Rule Codes

See [ARCHITECTURE.md](../../ARCHITECTURE.md) for the full specification.

## Rule Categories

| Prefix  | Category           | Description                          |
| ------- | ------------------ | ------------------------------------ |
| AES1xx  | Naming             | Suffix/prefix conventions per layer  |
| AES2xx  | Imports            | Mandatory, forbidden, unused, cycles |
| AES3xx  | Code Analysis      | Lines, bypasses, mandatory defs      |
| AES4xx  | Role/Layer Rules   | Layer-role violations                |
| AES5xx  | Orphan Detection   | Dead code via import graph           |

## Naming Suffixes by Layer

| Layer        | Allowed Suffixes                        |
| ------------ | --------------------------------------- |
| taxonomy     | `vo`, `constants`, `errors`, `events`   |
| contract     | `protocol`, `aggregate`                 |
| utility      | _(none — free functions only)_          |
| capabilities | `analyzer`, `checker`, `handler`        |
| agent        | `orchestrator`                          |
| surface      | `command`, `transport`                  |

## Import Rules

- `taxonomy_*` → taxonomy only
- `contract_*` → taxonomy, contract
- `utility_*` → taxonomy, contract, utility
- `capabilities_*` → taxonomy, contract, utility, capabilities
- `agent_*` → taxonomy, contract, agent
- `surface_*` → taxonomy, contract, utility, surface
