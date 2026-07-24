# AES Migration Guide — Python

> Skill-driven migration workflow for Python projects to AES architecture.
> Each phase delegates to a dedicated skill in `.agents/skills/`.

See [ARCHITECTURE.md](ARCHITECTURE.md) for layer rules and [README.md](README.md) for project usage.

## Workspace Structure

```
project-root/
├── pyproject.toml           ← workspace root config
├── modules/
│   ├── shared/              ← shared taxonomy + contract + utility types
│   │   ├── pyproject.toml
│   │   └── src/
│   │       ├── __init__.py
│   │       ├── common/          ← truly shared across ALL features
│   │       └── <feature>/       ← shared types per feature domain
│   │
│   ├── <feature>/          ← feature module
│   │   ├── pyproject.toml
│   │   └── src/
│   │       ├── __init__.py
│   │       ├── capabilities_<concept>_<role>.py
│   │       ├── agent_<concept>_orchestrator.py
│   │       ├── surface_<concept>_<role>.py
│   │       └── root_<concept>_container.py
│   └── ...
└── src/
    └── root_<name>_entry.py   ← entry point (at workspace root)
```

**Key rules:**

- All 7 layers coexist in each feature slice.
- Stable domain taxonomy, contracts, and utilities live under `modules/shared/src/<feature>/`.
- Orchestration, capabilities, and surfaces live in the feature module.
- Entry points (`root_*_entry.py`) live at workspace root or `src/`.
- Shared types go in `modules/shared/`.

---

## Prerequisites

```bash
pip install lint-arwaky-cli
lint-arwaky-cli version
lint-arwaky-cli scan your-project/
```

---

## Phase 0: Audit

> **Skill:** `lint-arwaky-python` — load for audit commands and violation analysis.

```bash
lint-arwaky-cli scan your-project/
find your-project/modules -name "*.py" | wc -l
```

- Violations < 10 → full migration in one session
- Violations 10-50 → phased migration
- Violations > 50 → start with taxonomy only

---

## Phase 1: Taxonomy Layer

> **Skill:** `create-taxonomy-python` — load for VOs, errors, constants, entities, events.

Define Value Objects, Errors, Events, and compile-time Constants under `modules/shared/src/<feature>/`.

### Steps

1. Identify domain types with `grep -rn "^class " modules/*/src/ | grep -v test | grep -v __init__`
2. Load `create-taxonomy-python` skill
3. Create taxonomy files following skill templates and workflow
4. Register in domain `__init__.py`
5. Verify: `python -c "import modules.shared.src.<feature>"`

---

## Phase 2: Contract Layer

> **Skill:** `create-contract-python` — load for protocol and aggregate ABCs.

Contracts define public interfaces (Protocols and Aggregates) without exposing implementation.

### Steps

1. Load `create-contract-python` skill
2. Create protocol ABCs (inbound/outbound) under `modules/shared/src/<feature>/`
3. Create aggregate facade ABCs under `modules/shared/src/<feature>/`
4. Register in domain `__init__.py`
5. Verify: `python -c "import modules.shared.src.<feature>"`

---

## Phase 3: Utility Layer

> **Skill:** `create-utility-python` — load for stateless standalone functions.

Utility contains low-level technical mechanics — **stateless standalone functions only**.

### Steps

1. Identify reusable stateless functions across modules
2. Load `create-utility-python` skill
3. Create utility files under `modules/shared/src/<feature>/`
4. Register in domain `__init__.py`
5. Verify: `python -c "import modules.shared.src.<feature>"`

---

## Phase 4: Capabilities Layer

> **Skill:** `create-capabilities-python` — load for business logic and external adaptation.

Capabilities contain concrete behavior implementations (business logic + external adapters).

### Steps

1. Load `create-capabilities-python` skill
2. Create business logic capabilities (implement protocol ABCs)
3. Create external adaptation capabilities (repositories, clients)
4. Verify: `python -c "import modules.user.src.capabilities_*"`

---

## Phase 5: Agent Layer

> **Skill:** `create-agent-python` — load for orchestration logic.

Orchestrates sequential execution, branching, looping, and error handling.

### Steps

1. Load `create-agent-python` skill
2. Create orchestrator class implementing aggregate ABC
3. Inject protocol dependencies via constructor
4. Verify: `python -c "import modules.user.src.agent_*"`

---

## Phase 6: Surface Layer

> **Skill:** `create-surface-python` — load for user-facing input translation.

Translates user-facing inputs into actions, delegating to the Agent orchestrator.

### Steps

1. Load `create-surface-python` skill
2. Create surface classes (commands, handlers, endpoints)
3. Inject aggregate ABC via constructor
4. Verify: `python -c "import modules.user.src.surface_*"`

---

## Phase 7: Root Layer

> **Skill:** `create-root-python` — load for DI container and entry point wiring.

Wires concrete implementations to contracts and bootstraps the system.

### Steps

1. Load `create-root-python` skill
2. Create DI container wiring all capabilities → orchestrator → surface
3. Create entry point at workspace root or `src/`
4. Verify: `python -c "import src.root_*_entry"`

---

## Phase 8: Verify

> **Skill:** `build-verify-all` — load for final build verification.

```bash
lint-arwaky-cli scan your-project/
pytest
ruff check . && ruff format --check .
```

---

## Supplementary Skills (Post-Migration)

| Skill | When to Use |
|-------|-------------|
| `add-docs-python` | Add docstrings, type hints after migration |
| `fix-bypass-python` | Remove `# type: ignore`, `noqa` |
| `cleanup-consolidate-python` | Remove dead code, merge duplicates |
| `create-test-python` | Generate test suites |

---

## Reference: File Naming & Import Rules

See [ARCHITECTURE.md](ARCHITECTURE.md) §3 (Naming Convention) and §11 (Import Rules).

| Layer | Pattern |
|-------|---------|
| taxonomy | `taxonomy_<concept>_<suffix>.py` |
| contract | `contract_<concept>_<suffix>.py` |
| utility | `utility_<concept>_<suffix>.py` |
| capabilities | `capabilities_<concept>_<suffix>.py` |
| agent | `agent_<concept>_orchestrator.py` |
| surface | `surface_<concept>_<suffix>.py` |
| root | `root_<concept>_<suffix>.py` |

---

## Troubleshooting

See [ARCHITECTURE.md](ARCHITECTURE.md) §12 (Troubleshooting) for violation codes and fixes.
