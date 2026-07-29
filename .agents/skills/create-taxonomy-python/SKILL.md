---
name: create-taxonomy-python
description: "Create and validate Python taxonomy layer files in shared taxonomy: VOs, entities, errors, events, and constants. Taxonomy is the domain foundation layer — stable language of the domain, free from technical or behavioral concerns."
metadata:
  tags: [python, aes, taxonomy, shared, vo, entity, error, event, constant, primitive-to-vo]
  triggers:
    - "create taxonomy python"
    - "add taxonomy python"
    - "move dataclass to taxonomy python"
    - "create vo python"
    - "create error taxonomy python"
    - "create constant taxonomy python"
    - "check taxonomy python"
    - "audit taxonomy python"
  dependencies: []
  related:
    - create-capabilities-python
    - create-agent-python
    - create-contract-python
---

# create-taxonomy-python

Taxonomy = **stable language of the domain**. Single source of truth for VOs, entities, errors, events, constants. Free from technical/behavioral concerns. Location: `modules/shared/src/<domain>/`.

## Taxonomy Types

| File Suffix | Content | Rules |
| --- | --- | --- |
| `_vo.py` | Value Objects | Validate on construction, immutable, no I/O |
| `_entity.py` | Entities with identity | Identity field required (VO), no I/O |
| `_error.py` | Domain error types | Extend `Exception`, VO fields only |
| `_event.py` | Domain event types | Immutable, VO payload fields |
| `_constant.py` | Compile-time constants | Pure literals only, no functions, no I/O |
| `_utility.py` | Stateless helper functions | No class, no self, pure, domain-agnostic |

## Definition of Done

1. Correct file suffix (`_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`).
2. VOs validate on construction when invariants exist.
3. No raw primitives in fields — use other VOs.
4. No I/O, no side effects, no business logic in VOs/entities/errors/events/constants.
5. Constants: pure static literal values only.
6. Taxonomy imports only other taxonomy types or stdlib.
7. No import from capabilities, agents, surface, root, contracts.
8. Registered in shared `__init__.py`.
9. `python -c "import <module>"` passes.

---

## Purity and Import Restrictions (AES201/AES401)

Taxonomy must remain pure and stable.

| Taxonomy Type | May Import From | Must Not Import From |
| --- | --- | --- |
| `_vo`, `_entity`, `_error`, `_event` | other taxonomy types, stdlib | capabilities, agents, surface, root, contracts, I/O |
| `_constant` | only core/static values | external layer imports, I/O, functions |

**Taxonomy MAY contain:** value validation, domain invariants in constructors, pure transformations between taxonomy types.

**Taxonomy MUST NOT contain:** file I/O, network calls, database access, env mutation, side effects, business orchestration, use-case logic.

---

## VO Rules (AES401/AES402)

Domain data MUST use VOs, not raw primitives.

| Primitive | Rule |
| --- | --- |
| `str`, `int`, `float` | Forbidden for domain fields. Use VO. |
| `bool` | Allowed for semantic toggles only. |
| `list[str]`, `dict` | Forbidden for domain collections. Use VO. |

Prefer VOs for: file paths, symbol names, messages, line numbers, severity, counts, thresholds, identifiers, results, policies.

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/taxonomy_name_vo.py` | Value Object template |
| `templates/taxonomy_name_entity.py` | Entity template |
| `templates/taxonomy_name_error.py` | Error type template |
| `templates/taxonomy_name_constant.py` | Constants template |

---

## Workflow

1. **Determine type** — VO / Entity / Error / Event / Constant / Utility?
2. **Create file** → `taxonomy_<domain>_<type>.py` in `shared/src/<domain>/`.
3. **VOs**: validate in `__init__`, use `@dataclass(frozen=True)` or manual immutability.
4. **Entities**: add identity VO field.
5. **Errors**: extend `Exception`, use VO fields.
6. **Constants**: pure literals, no functions.
7. **Register** → update `__init__.py`.
8. **Verify** → `python -c "import <module>"`.

---

## Verification Checklist

- [ ] Correct file suffix.
- [ ] VOs validate on construction when invariants exist.
- [ ] Single-value VOs expose safe constructors and accessors.
- [ ] Composite VOs use other VOs instead of raw primitives.
- [ ] Error types extend `Exception`.
- [ ] Constants are pure literal values.
- [ ] No import from capabilities, agents, surface, root, contracts.
- [ ] No I/O, no network, no database in taxonomy files.
- [ ] Registered in shared `__init__.py`.
- [ ] `python -c "import <module>"` passes.
