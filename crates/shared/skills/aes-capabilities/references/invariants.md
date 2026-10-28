# aes-capabilities — Invariants

Every rule is machine-checked by `lint-arwaky-cli scan <layer-path>`.

| Rule | Detail |
|------|--------|
| Naming | File `capabilities_<domain>_<role>` — role from internal/external naming lists; forbidden suffixes `_vo`/`_entity`/`_error`/`_event`/`_constant`/`_constants`/`_protocol`/`_aggregate`/`_utility`/`_request`/`_response` (AES101/AES102). |
| Structure | 3-block order: Block 1 type+ctor → Block 2 protocol methods only → Block 3 factories/dunders/helpers. ≥1 protocol implementor, ≤3 types (AES403). |
| Imports | Taxonomy + `_protocol` contracts + utility only — never agents, siblings, surface, local domain models (AES201–AES205). |
| DI | Protocol interfaces only (`Arc<dyn Trait>` in Rust); shared VOs in fields/signatures. |
| Helpers | Stateless domain-agnostic ≥2-consumer functions move to Utility; constants to `taxonomy_*_constant`. |
| Register | Shared barrel: `__init__.py` / `mod.rs` / `index.ts`. |
| Verify | `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only. |

## Placement (AES702, AES703)

A `capabilities_*` file belongs in a **feature folder** — a subdirectory of a member directory (`crates/`, `modules/`, `packages/`) that is neither `shared/` nor a surface folder:

```
crates/<feature_name>/src/capabilities_<domain>_<role>.*    # Rust
modules/<feature_name>/src/capabilities_<domain>_<role>.py  # Python
packages/<feature_name>/src/capabilities_<domain>_<role>.ts # TypeScript
```

The prefix is `capabilities_` in every language. A file named `capability_x.py` is not a capability file to the linter.

Every feature folder needs **at least one `capabilities_*` file and at least one `agent_*_orchestrator` file**. A feature folder with only capabilities has nothing orchestrating them — an **AES702** violation.

`capabilities_*` files are forbidden in two places:

- **`shared/`** — the shared folder holds taxonomy, utility, and contract files only (**AES701**).
- **A surface folder** — a folder where surface files outnumber everything else carries surface files only (**AES703**).

### Feature folder documents

A feature folder carries two documents at its root, beside the source:

```
crates/<feature_name>/FRD.md       # what the feature does — requirements
crates/<feature_name>/BACKLOG.md   # where its work stands
```

The folder name matches the feature it serves: `crates/calculator/` documents the calculator, `modules/naming_violations/` documents naming violations. A folder named after a generic word (`utils`, `common`, `core`) describes no feature and cannot carry a meaningful FRD.

A feature folder carrying capabilities or an agent but neither document is an **AES702** violation — the linter names the missing file and you write it. A folder carrying neither capabilities nor an agent is not a feature and owes no document pair.

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is this protocol behaviour (not a stateless helper or pure function)?**
   - *No* → it is a utility, not a capability. See `aes-utility` skill.
2. **Does a `_protocol` contract exist for this behaviour?**
   - *No* → create it first with `aes-contract`.
3. **Block 2 contain only protocol methods? ≥1 implementor? ≤3 types?**
   - *No* → restructure to 1→2→3; split or extract helpers.
4. **Any forbidden imports or local domain models?**
   - *Yes* → strip; shared needs go through contract or root.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.

## Workflow

1. Confirm the work is protocol behaviour (not orchestration / data / mechanics).
2. Ensure a `_protocol` contract exists — if missing, run `aes-contract` first.
3. Resolve the feature capability dir. Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
4. Implement the 3 blocks from the language HOW-TO § Template / § Section Contract.
5. Strip forbidden imports / inter-capability deps / local domain models; move constants and pure helpers out.
6. Register in the shared barrel, verify with `lint-arwaky-cli scan`, then wire via `aes-agent` / `aes-root`.

## Verification

### Machine Checks

```bash
lint-arwaky-cli scan <layer-path>   # AES101/102, AES201–205, AES401–406 → must be 0
# Fallback only: language compile (python -c import / cargo check / npx tsc --noEmit)
```

### Human Checks

A machine pass does not mean the file is right. Layer purpose, structural order, and "only what this layer may do" still need a reader (HOW-TO § Rules).

## Pre-flight Checklist

- [ ] `lint-arwaky-cli scan <layer-path>` exits 0.
- [ ] Every touched HOW-TO's `Verify` block was executed.
- [ ] File registered in `__init__.py` / `mod.rs` / `index.ts`.
- [ ] Language fallback compile clean if the HOW-TO lists it.
- [ ] Related skills considered for the next layer up/down.

## Common Mistakes (Anti-Patterns)

- **Stateless helper with no protocol behaviour in a capability file**: extract to `utility_*` — it belongs in shared, not the feature folder.
- **Business logic mixed with a pure helper**: split — business rules stay; stateless logic moves to utility.
- **Capabilities importing each other**: shared needs go through contract or root, not sibling imports.
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.
