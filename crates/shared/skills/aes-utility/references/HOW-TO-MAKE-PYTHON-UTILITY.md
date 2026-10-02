# HOW TO MAKE UTILITY PYTHON

> **Purpose**: Hold stateless, domain-agnostic helper functions shared across the system.
>
> **Audience**: Agents and engineers scaffolding AES utility files in the shared domain.
>
> **Scope**: Python, Rust, and TypeScript `utility_<domain>_<role>` files — free functions only, no class/struct/impl.
>
> **Location**: Shared domain source root next to taxonomy, registered in the shared barrel.
>
> **Length**: Free functions only; used by ≥2 modules; taxonomy-only imports.

---

## Rules

### Utility vs Capability Boundary

A utility file **performs a narrow, stateless task** — it has no business rules, no injected dependencies, no instance state.
A capability file **implements a protocol** — it has DI, business rules, and concrete behaviour.

| Decision | → Utility | → Capability |
|---|---|---|
| Has `self` / instance state / DI params | No | Yes |
| Contains business rules or protocol logic | No | Yes |
| Stateless, pure, domain-agnostic, ≥2 consumers | Yes | No |
| Implements `_protocol` interface/ABC/trait | No | Yes |
| Performs a narrow reusable operation (e.g., string parsing, date formatting) | Yes | No |

**Rule**: If a function uses `self`, contains business rules, or serves a single consumer — it does not belong in a utility file. Move it to the consuming capability or keep it as a private helper.

### Import rules

**Allowed imports:** Taxonomy only.
**Forbidden:** Capabilities, Agent, Surface, Contract, and other `utility_*` files.
The last one is a cross-file rule in all three languages: a utility importing another
utility couples two helpers that should be independently extractable.

### Structure rules (Python)

1. **Structure:** Only module-level functions — no `class`, no `self`.
2. **Purity:** Pure + deterministic — no `random`, no `datetime.now()`, no global mutable state.
3. **Domain Awareness:** Domain-agnostic — no business rules, no layer-name knowledge.
4. **Reusability:** Used by ≥2 modules; if single consumer → keep as private helper.
5. **I/O Constraint:** I/O allowed only if all above hold.

**Keep as private helper** if ANY: uses `self`, domain-specific, single consumer.
**Extract here** only if ALL: no `self`, pure/I/O-safe, domain-agnostic, ≥2 consumers.

### Workflow

1. Confirm ≥2 consumers, stateless, domain-agnostic.
2. Create `utility_<domain>_<role>.py`.
3. Register in `__init__.py`.
4. `python -c "import <module>"`.

---

## Template

```python
"""<Domain> utility functions — stateless, pure, domain-agnostic.

Module-level functions only — no classes, no state.
"""

# from shared.user.taxonomy_user_vo import UserVO  # uncomment if using VOs

def <function_name>(<param_name>: str) -> str:
    """<Description of what this function does>.

    Args:
        <param_name>: <description>

    Returns:
        <description of return value>
    """
    # pure function logic here
    pass


def <second_function_name>(<param_name>: str) -> str:
    """<Description of what this second function does>.

    Args:
        <param_name>: <description>

    Returns:
        <description of return value>
    """
    # pure function logic here
    pass
```

---

## Section Contract

| Check | Why it belongs here |
| ----- | ------------------- |
| Only module-level functions — no class. | Required by AES layer rules and the linter; missing it is a defect. |
| No `self`, no instance state. | Required by AES layer rules and the linter; missing it is a defect. |
| Pure/deterministic (or I/O justified: domain-agnostic + reusable). | Required by AES layer rules and the linter; missing it is a defect. |
| No business rules or layer-name knowledge. | Required by AES layer rules and the linter; missing it is a defect. |
| Used by ≥2 modules. | Required by AES layer rules and the linter; missing it is a defect. |
| No import from Capabilities, Agent, Surface, Contract. | Required by AES layer rules and the linter; missing it is a defect. |
| No magic constants (→ `taxonomy_*_constant.py`). | Required by AES layer rules and the linter; missing it is a defect. |
| `python -c "import <module>"` passes. | Required by AES layer rules and the linter; missing it is a defect. |

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Machine-checked: class declarations (metadata path), naming, imports, primitives.
# Manual (not machine-checked): stateless, domain-agnostic, ≥2 consumers; no class/`self`/`this`/struct/impl.
# Fallback compile gate: python -c "import <shared_package>.<module>"
```
