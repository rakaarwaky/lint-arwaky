---
name: create-utility-python
description: "Create and validate Python utility layer files following AES rules: stateless standalone functions, no class, no protocol impl, pure functions, domain-agnostic, reusable across modules."
metadata:
  tags: [python, aes, utility, shared, stateless, pure-function, domain-agnostic]
  triggers:
    - "create utility python"
    - "add utility python"
    - "extract utility python"
    - "create helper function python"
    - "check utility python"
    - "audit utility python"
  dependencies: []
  related:
    - create-taxonomy-python
    - create-capabilities-python
    - create-agent-python
---

# create-utility-python

Utility layer = **stateless standalone functions**. No class, no `self`, no domain rules. Pure, domain-agnostic, reusable. File: `utility_<domain>_<role>.py` (or `taxonomy_<domain>_utility.py` in shared).

## Role Naming

parser, splitter, trimmer, slugifier, sanitizer, normalizer, extractor, replacer, converter, counter, resolver, detector, builder, joiner, serializer, deserializer, encoder, decoder, hasher, generator, formatter, comparator, differ, matcher, checker, calculator, mapper, merger, grouper, sorter, deduplicator, printer

## Definition of Done

1. Only module-level functions — no class definitions.
2. No `self` parameter, no class attributes, no instance state.
3. Pure: same input → same output (except I/O utilities).
4. Domain-agnostic: no business rules, no architecture knowledge.
5. Reusable: used by ≥2 modules (otherwise keep as private helper).
6. I/O allowed only if stateless + domain-agnostic + reusable.
7. No import from Capabilities, Agent, Surface, Contract.
8. May import from Taxonomy only.
9. `python -c "import <module>"` passes.

---

## Stateless Rules

1. **No classes** — no `class`, no `self`, no instance state.
2. **Pure functions** — deterministic: same input → same output. No `random`, no `datetime.now()`, no global mutable state.
3. **Domain-agnostic** — must NOT know about: architecture layer names (agent, capabilities, contract), business domain rules, specific capability logic.
4. **Reusable** — if only one module uses it → keep as private helper in that module.

### I/O Exception

Utility CAN perform I/O if ALL conditions met: stateless (no `self`), domain-agnostic, reusable across multiple modules.

---

## Keep vs Extract Decision

**Keep as private helper** if ANY: accesses `self`/instance state, domain-specific, only one consumer.

**Extract to utility** only if ALL: stateless, pure (I/O allowed), domain-agnostic, ≥2 consumers.

---

## Layer Boundaries

| Allowed | Forbidden |
| --- | --- |
| Stateless module-level functions | Class definitions |
| Pure computation (input → output) | `self` / instance state |
| I/O (if domain-agnostic + reusable) | Business rules / domain knowledge |
| Taxonomy imports (`shared.taxonomy_*`) | Import from Capabilities, Agent, Surface |
| File walking, pattern matching, parsing | Protocol/aggregate implementation |
| Environment access (if stateless + reusable) | Magic constants (→ `taxonomy_*_constant.py`) |

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/utility_name.py` | Utility function module template |

---

## Workflow

1. **Confirm reusability** — Is this used by ≥2 modules? If no → keep as private helper.
2. **Confirm stateless** — No `self`, no class, no global mutation?
3. **Confirm domain-agnostic** — No business rules, no architecture knowledge?
4. **Create file** → `utility_<domain>_<role>.py`.
5. **Register** → update `__init__.py`.
6. **Verify** → `python -c "import <module>"`.

---

## Verification Checklist

- [ ] Only module-level functions — no class definitions.
- [ ] No `self`, no instance state.
- [ ] Pure / deterministic (or I/O with domain-agnostic + reusable justification).
- [ ] Domain-agnostic — no business rules, no layer-name knowledge.
- [ ] Used by ≥2 modules (otherwise keep as private helper).
- [ ] No import from Capabilities, Agent, Surface, Contract.
- [ ] No magic constants (extracted to `taxonomy_*_constant.py`).
- [ ] `python -c "import <module>"` passes.
