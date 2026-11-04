# Phase gates

Every rule is machine-checked by `lint-arwaky-cli scan <project-root>` (see each language playbook § Verify). A rule cannot drift from the gate. Cite the linter, not this file, when pointing at a rule.

| Phase | Rule | Gate |
| ----- | ---- | ---- |
| 0 — Audit | Scan before touching anything; record baseline violations. | `lint-arwaky-cli scan .` |
| 1 — Taxonomy | Value objects, errors, events, constants only; no I/O; no upward imports (AES201). | `lint-arwaky-cli scan <taxonomy-dir>` |
| 2 — Contract | Protocol = one file per feature, one trait per capability seam (each rich); aggregate = exactly one `execute()` method. | `lint-arwaky-cli scan <contract-dir>` |
| 3 — Utility | Stateless free functions; taxonomy-only imports; ≥2 consumers (else keep private). | `lint-arwaky-cli scan <utility-dir>` |
| 4 — Capabilities | 3-block structure; ≥1 protocol implementor; ≤3 type declarations per file (AES403). | `lint-arwaky-cli scan <capabilities-dir>` |
| 5 — Agent | Implements aggregate; DI via protocol types (`Arc<dyn Trait>` / constructor injection); ≤3 types (AES405). | `lint-arwaky-cli scan <agent-dir>` |
| 6 — Surface | Imports aggregate (not agent/capabilities); call-site only for contracts (AES201 purpose). | `lint-arwaky-cli scan <surface-dir>` |
| 7 — Root | Wires capabilities → agent → surface; no business logic; entry bootstrap only. | `lint-arwaky-cli scan <root-dir>` |
| 8 — Verify | 0 violations across all layers; language compile clean; tests pass. | `lint-arwaky-cli scan .` → 0; `python -m compileall` / `cargo check` / `npx tsc --noEmit` |

Split import matrices, phase templates, and troubleshooting tables: **read the language playbook** — do not restate them here.

## Dependency model

```text
root ── wires ──→ agent / surface / capabilities
                  │               │              │
                  ▼               ▼              ▼
            contract (protocol / aggregate)
                  │
                  ▼
            taxonomy (VOs, entities, errors, events, constants)
                  ▲
            utility ── imports only taxonomy ──┘
```
