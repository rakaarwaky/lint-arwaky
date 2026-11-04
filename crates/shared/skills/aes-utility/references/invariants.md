# Invariants

Every rule is machine-checked by `lint-arwaky-cli scan <layer-path>` (see each HOW-TO § Verify). A rule cannot drift from the gate; cite the linter, not this file, when pointing at a rule.

| Layer | Rule |
| ----- | ---- |
| Naming | File `utility_<domain>_<role>` — suffix flexible; forbidden: `_vo`/`_entity`/`_error`/`_event`/`_constant`/`_protocol`/`_aggregate` (AES101/AES102). |
| Structure | Free / module-level / exported functions only — no class, no `struct`/`impl`/trait/`type` alias (Rust), no `interface`/`enum`/`type` alias (TS), no `self`/`this` (AES404). |
| State | Stateless and deterministic — no `random`/`now()`/`Math.random()`/`Date.now()`, no global mutable state. |
| Domain | Domain-agnostic — no business rules, no layer-name knowledge; ≥2 consumers (else keep as a private helper). |
| Imports | Taxonomy only — never capabilities, agent, surface, contract, or other `utility_*` files (AES201). |
| Register | Shared barrel: `__init__.py` / `mod.rs` / `index.ts`. |
| Verify | `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only. |

Split details, templates, and Section Contract tables: read the language HOW-TO — do not restate them here.
