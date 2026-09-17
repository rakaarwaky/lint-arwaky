# AES Rule → Remediation → Skill Routing

Authoritative mapping for every AES code emitted by `lint-arwaky-cli scan`. Rule semantics come
from `internal/lint-arwaky/RULES_AES.md` and `internal/lint-arwaky/lint_arwaky.config.yaml` —
those files are the source of truth if this table and the code disagree.

Triage order when a scan reports many violations:
🔴 **CRITICAL** AES201, AES205, AES304 → 🟡 **HIGH** AES101–102, AES202, AES301–303,
AES401–403, AES406, AES505–506 → 🟢 **MEDIUM/LOW** AES203–204, AES305, AES404–405, AES501–504.

## Fix table

| Rule (name)                    | What it checks                                                                 | Fix approach                                        | Skill to use |
| -------------------------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------ | -------------- |
| AES101 (Naming Convention)     | Filename is `<layer>_<concern>_<role>.<ext>`, lowercase, ≥3 words              | Rename, then update barrel/`mod.rs`/`__init__.py` | `create-{layer}` → `references/<lang>.md` |
| AES102 (Suffix/Prefix Rules)   | Final suffix is legal for the layer (matrix below)                             | Change suffix (and therefore the role) to a legal one | `create-{layer}` |
| AES201 (Forbidden Import)      | A layer imports a layer it must not (matrix below)                             | Remove the cross-layer import; depend on a contract protocol/aggregate injected via DI | `create-contract` |
| AES202 (Mandatory Import)      | A required import is missing (e.g. capabilities must import `contract(*_protocol)`) | Add the required import/type                    | `create-{layer}` |
| AES203 (Unused Import)         | Imported symbol never used                                                     | `lint-arwaky-cli fix <path> --filter AES203`      | — |
| AES204 (Dummy Import)          | Import kept only to satisfy a linter, with stub usage                          | Remove the dummy import and its stub              | `fix-bypass` |
| AES205 (Circular Import)       | Two files/layers import each other                                             | Extract the shared type or trait downward into `taxonomy` or `contract`, both sides import that | `create-contract` |
| AES301 (File Maximum Limit)    | File exceeds the layer's max lines                                             | Split by responsibility                           | `cleanup-consolidate` |
| AES302 (File Minimum Limit)     | File is below the minimum size                                                 | Merge into the parent module or delete            | `cleanup-consolidate` |
| AES303 (Mandatory Definition)  | File has no class/struct/enum/trait of its own                                 | Add the definition the filename promises        | `create-{layer}` |
| AES304 (Bypass Comment)        | `noqa`, `type: ignore`, `#[allow]`, `@ts-ignore`, `eslint-disable` …        | Fix the root cause and delete the suppression     | `fix-bypass` |
| AES305 (Duplication Code)      | Same logic repeated across files                                               | Extract shared logic into a pure utility          | `create-utility` |
| AES401 (Taxonomy Role)         | `_constant` purity; primitives in `_entity`/`_error`/`_event`             | Replace primitives with taxonomy VOs              | `create-taxonomy` |
| AES402 (Contract Role)         | Contract trait/method signatures use primitives instead of taxonomy VO/constant | Replace primitives with VO/constant types       | `create-contract` |
| AES403 (Capabilities Role)     | >3 type declarations; no implementor of the capability protocol               | Implement the protocol; split routing across capabilities | `create-capabilities` |
| AES404 (Utility Role)          | Utility must be stateless standalone functions and may import taxonomy only     | Move stateful logic to capabilities; drop non-taxonomy imports | `create-utility` |
| AES405 (Agent Role)            | Agent: >3 types, no aggregate implementor, `Any` annotations, direct capabilities import, state outside the constructor | Depend on `contract(*_aggregate)` and delegate to capabilities via protocols | `create-agent` |
| AES406 (Surface Role)          | Surface >15 functions; business logic in a passive surface; role-boundary breach | Move logic down a layer; keep passive surfaces declarative | `create-surface` |
| AES501–506 (Orphan per layer)  | A taxonomy/utility/contract/capabilities/agent/surface file nothing imports     | Wire into the root container or delete dead code  | `cleanup-consolidate`, `create-root` |

`{layer}` ∈ `taxonomy`, `utility`, `contract`, `capabilities`, `agent`, `surface`, `root` — the
merged skill name (e.g. `create-contract`), whose `references/<language>.md` carries the code
templates for Python, Rust, or TypeScript.

## AES102 — legal suffixes per layer

| Layer        | Suffix policy |
| ---------------- | ------------------------------------------------------- |
| `taxonomy`     | strict: `vo`, `entity`, `error`, `event`, `constant` |
| `utility`      | flexible; **forbidden**: any taxonomy suffix, `protocol`, `aggregate` |
| `contract`     | strict: `protocol`, `aggregate`                      |
| `capabilities` | flexible; **forbidden**: any taxonomy suffix, `constants`, `protocol`, `aggregate`, `utility` |
| `agent`        | strict: `orchestrator`                               |
| `surface`      | strict: `command`, `controller`, `page`, `view`, `component`, `router`, `layout`, `hook`, `store`, `action`, `screen` — **not** `entry` |
| `root`         | strict: `entry`, `container`                         |

## AES201 — import matrix (allowed / mandatory / forbidden)

| Scope of the file                | Allowed imports | Mandatory | Forbidden |
| ------------------------------------ | ------------------------------------------------------- | -------------------------------- | ------------------------------------------------- |
| `taxonomy(vo)`, `taxonomy(constant)` | `taxonomy`                                          | —                             | everything else |
| `taxonomy(entity,error,event)`       | `taxonomy`                                          | `taxonomy(vo\|constant)` | everything else |
| `utility`                            | `taxonomy`                                          | —                             | every other layer **and other `utility`** |
| `contract(protocol)`                 | `taxonomy`, `contract`                            | `taxonomy` | `capabilities`, `agent`, `surface`, `root`, `contract(aggregate)` |
| `contract(aggregate)`                | `taxonomy`, `contract`                            | `taxonomy` | `capabilities`, `agent`, `surface`, `root`; may not *inherit* `contract(protocol)` |
| `capabilities`                       | `taxonomy`, `contract`, `utility`                     | `taxonomy`, `contract(protocol)` | other `capabilities`, `agent`, `surface`, `root` |
| `agent(orchestrator)`                | `taxonomy`, `contract(aggregate)`, `contract(protocol)`, `utility` | `taxonomy`, `contract(aggregate)` | `capabilities`, `surface`, `root` |
| `surface(command\|controller\|page)`  | `taxonomy`, `contract(aggregate)`, `utility`          | —                             | `agent`, `capabilities`, `contract(protocol)`, `root` |
| `surface(hook\|store\|action\|screen\|router)` | `taxonomy` only                              | —                             | all other layers, plus `surface(command\|controller\|page\|entry)` |
| `surface(component\|view\|layout)`     | `taxonomy` only                                      | —                             | all other layers, plus every non-passive surface |
| `root`                               | `taxonomy`, `contract`, `capabilities`, `agent`, `surface` | —                        | — (top layer) |

Files under `tests/` and `benches/` are **not** an AES layer — the `test` prefix is not a layer
and no AES102 suffix rule applies to it. Use the `testing-suite` skill for test naming,
layout, and run commands.
