# Placement (AES702, AES703)

Place every `agent_*_orchestrator` file in the feature folder the linter expects, with its FRD/BACKLOG document pair beside the source.

An `agent_*_orchestrator` file belongs in a **feature folder** — a subdirectory of a member
directory (`crates/`, `modules/`, `packages/`) that is neither `shared/` nor a surface folder:

```text
crates/<feature_name>/src/agent_<domain>_orchestrator.rs    # Rust
modules/<feature_name>/src/agent_<domain>_orchestrator.py  # Python
packages/<feature_name>/src/agent_<domain>_orchestrator.ts # TypeScript
```

A feature folder is defined as a folder holding at least one `agent_*_orchestrator` and one
`capabilities_*` file. If a folder holds only capabilities without an agent of its own it is
an **AES702** violation; if the agent sits in a surface-dominated folder it is also **AES702**
and the file should be moved out of the surface folder.

An orchestrator may sit at the **member root** (e.g. `packages/agent_calculator_orchestrator.ts`)
when it routes across features rather than serving one. That makes it a routing aggregate
over the features beneath it — it holds one orchestrator per feature and delegates to them.
It does **not** satisfy AES702 for those folders: each feature still owes an orchestrator of
its own, so a capabilities folder under a member-level orchestrator is still a split feature.

`agent_*_orchestrator` files are forbidden in `shared/` — that is an **AES701** violation and
the file must be moved to a feature folder.

## Feature folder documents

The feature folder holding this agent also carries two documents at its root, beside the
source:

```text
crates/<feature_name>/FRD.md       # what the feature does — requirements
crates/<feature_name>/BACKLOG.md   # where its work stands
```

The folder name matches the feature it serves. A folder named after a generic word
(`utils`, `common`, `core`) describes no feature and cannot carry a meaningful FRD.

A feature folder carrying an agent but neither document is an **AES702** violation — the
linter names the missing file and you write it.

---

## Completion

Placement is settled when:

- `lint-arwaky-cli scan <layer-path>` reports no AES702 or AES703 findings.
- The feature folder root carries `FRD.md` and `BACKLOG.md` beside the source.
- No `agent_*_orchestrator` file sits in `shared/`.
