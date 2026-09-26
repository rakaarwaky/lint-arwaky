# HOW TO USE LINT ROUTING

> **Purpose**: Map every AES code from `lint-arwaky-cli scan` to its fix approach and the
> `aes-*` skill that owns that layer.
>
> **Audience**: Agents triaging a multi-violation scan before applying fixes.
>
> **Scope**: AES101–AES506: fix table, legal suffixes per layer (AES102), import matrix
> (AES201).
>
> **Location**: Decision table consulted after a scan; rule semantics ultimately come from
> `internal/aes-lint-arwaky/RULES_AES.md` and `internal/aes-lint-arwaky/lint_arwaky.config.yaml` —
> those files win if this table and the code disagree.
>
> **Length**: Triage order + one row per AES code + two reference matrices (suffix, imports).

---

## Rules

Six rules. Each one prevents a specific failure mode.

1. **Triage CRITICAL before HIGH/MEDIUM.** 🔴 AES201, AES205, AES304 → 🟡 AES101–102, AES202,
   AES301–303, AES401–403, AES406, AES505–506 → 🟢 AES203–204, AES305, AES404–405, AES501–504.
2. **Route every fix to its owning skill.** `{layer}` ∈ `taxonomy`, `utility`, `contract`,
   `capabilities`, `agent`, `surface`, `root` — the merged skill name (e.g. `aes-contract`),
   whose HOW-TO carries the language templates.
3. **Auto-fix only AES101 / AES203 / AES304 via `fix`.** AES201 and AES205 are structural —
   manual, through `aes-contract` (or the layer's `aes-*`).
4. **After any AES101 rename, update the barrel.** Python `__init__.py`, Rust `mod.rs`,
   TypeScript `index.ts` — otherwise the rename becomes AES501–506.
5. **AES102 and AES201 are matrices, not vibes.** Check the suffix and import tables below
   before inventing a "close enough" role name or import.
6. **`test` / `bench` are not AES layers.** Files under `tests/` and `benches/` follow
   `aes-testing-suite` naming; no AES102 suffix rule applies.

---

## Template

Copy the triage order into the scan session; pick the row by AES code.

### Triage order

🔴 **CRITICAL** AES201, AES205, AES304 → 🟡 **HIGH** AES101–102, AES202, AES301–303,
AES401–403, AES406, AES505–506 → 🟢 **MEDIUM/LOW** AES203–204, AES305, AES404–405, AES501–504.

### Fix table

| Rule (name) | What it checks | Fix approach | Skill to use |
| ----------- | -------------- | ------------ | ------------ |
| AES101 (Naming Convention) | Filename is `<layer>_<concern>_<role>.<ext>`, lowercase, ≥3 words | Rename, then update barrel/`mod.rs`/`__init__.py` | `aes-{layer}` |
| AES102 (Suffix/Prefix Rules) | Final suffix is legal for the layer (matrix below) | Change suffix (and therefore the role) to a legal one | `aes-{layer}` |
| AES201 (Forbidden Import) | A layer imports a layer it must not (matrix below) | Remove the cross-layer import; depend on a contract protocol/aggregate injected via DI | `aes-contract` |
| AES202 (Mandatory Import) | A required import is missing (e.g. capabilities must import `contract(*_protocol)`) | Add the required import/type | `aes-{layer}` |
| AES203 (Unused Import) | Imported symbol never used | `lint-arwaky-cli fix <path> --filter AES203` | — |
| AES204 (Dummy Import) | Import kept only to satisfy a linter, with stub usage | Remove the dummy import and its stub | `fix-bypass` |
| AES205 (Circular Import) | Two files/layers import each other | Extract the shared type or trait downward into `taxonomy` or `contract`; both sides import that | `aes-contract` |
| AES301 (File Maximum Limit) | File exceeds the layer's max lines | Split by responsibility | `cleanup-consolidate` |
| AES302 (File Minimum Limit) | File is below the minimum size | Merge into the parent module or delete | `cleanup-consolidate` |
| AES303 (Mandatory Definition) | File has no class/struct/enum/trait of its own | Add the definition the filename promises | `aes-{layer}` |
| AES304 (Bypass Comment) | `noqa`, `type: ignore`, `#[allow]`, `@ts-ignore`, `eslint-disable` … | Fix the root cause and delete the suppression | `fix-bypass` |
| AES305 (Duplication Code) | Same logic repeated across files | Extract shared logic into a pure utility | `aes-utility` |
| AES401 (Taxonomy Role) | `_constant` purity; primitives in `_entity`/`_error`/`_event` | Replace primitives with taxonomy VOs | `aes-taxonomy` |
| AES402 (Contract Role) | Contract trait/method signatures use primitives instead of taxonomy VO/constant | Replace primitives with VO/constant types | `aes-contract` |
| AES403 (Capabilities Role) | >3 type declarations; no implementor of the capability protocol | Implement the protocol; split routing across capabilities | `aes-capabilities` |
| AES404 (Utility Role) | Utility must be stateless standalone functions and may import taxonomy only | Move stateful logic to capabilities; drop non-taxonomy imports | `aes-utility` |
| AES405 (Agent Role) | Agent: >3 types, no aggregate implementor, `Any` annotations, direct capabilities import, state outside the constructor | Depend on `contract(*_aggregate)` and delegate to capabilities via protocols | `aes-agent` |
| AES406 (Surface Role) | Surface >15 functions; business logic in a passive surface; role-boundary breach | Move logic down a layer; keep passive surfaces declarative | `aes-surface` |
| AES501–506 (Orphan per layer) | A taxonomy/utility/contract/capabilities/agent/surface file nothing imports | Wire into the root container or delete dead code | `cleanup-consolidate`, `aes-root` |

`{layer}` ∈ `taxonomy`, `utility`, `contract`, `capabilities`, `agent`, `surface`, `root` —
the merged skill name (e.g. `aes-contract`), whose HOW-TO carries the templates for Python,
Rust, or TypeScript.

---

## Section Contract

Every routing decision is required to carry the rows that apply. Each exists for one reason.

| Section | Why it belongs here |
| ------- | ------------------- |
| Triage order | Fixing cosmetics before AES201/205/304 wastes the session and re-opens merges. |
| Fix table row per AES code | Every emitted code has exactly one fix path and one owning skill. |
| AES102 suffix matrix | "Flexible" vs "strict" is layer-specific; guessing recreates AES102. |
| AES201 import matrix | Allowed / mandatory / forbidden is per file scope, not a blanket rule. |
| Barrel note after rename | AES101 without barrel update → AES501–506. |
| Non-layer note for tests | `tests/` / `benches/` are outside AES101–102. |
| Source-of-truth pointer | `RULES_AES.md` + `lint_arwaky.config.yaml` win on disagreement. |

---

## Verify

```bash
# After applying routed fixes — must be 0
lint-arwaky-cli scan <target-path>

# Optional: isolate families while iterating
lint-arwaky-cli import <path> --filter AES201
lint-arwaky-cli role <path> --filter AES403
lint-arwaky-cli naming <path>
lint-arwaky-cli orphan <path> --format json
```

Manual (routing itself is not machine-checked):

- [ ] Every CRITICAL code in the scan has a row in the fix table before you start fixing.
- [ ] Each fix went to the Skill listed for that AES code (not a hand-rolled edit).
- [ ] Barrel/`mod.rs`/`index.ts` updated after every AES101 rename.
- [ ] AES102 / AES201 checked against the matrices, not memory.
- [ ] Re-scan to 0; then language verify from the matching `HOW-TO-USE-LINT-<LANG>.md`.
