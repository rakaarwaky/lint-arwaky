# Utility — Python

File: `utility_<domain>_<role>.py`. Utility = stateless standalone functions. No class, no `self`, no domain rules.

## Import rules

**Allowed imports:** Taxonomy only.
**Forbidden:** Capabilities, Agent, Surface, Contract.

## Templates

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

def <function_name>(<param_name>: str) -> str:
    """<Description of what this function does>.

    Args:
        <param_name>: <description>

    Returns:
        <description of return value>
    """
    # pure function logic here
    pass
```

## Rules (Python)

1. Only module-level functions — no `class`, no `self`.
2. Pure + deterministic — no `random`, no `datetime.now()`, no global mutable state.
3. Domain-agnostic — no business rules, no layer-name knowledge.
4. Reusable — used by ≥2 modules; if single consumer → keep as private helper.
5. I/O allowed only if all above hold.

**Keep as private helper** if ANY: uses `self`, domain-specific, single consumer.
**Extract here** only if ALL: no `self`, pure/I/O-safe, domain-agnostic, ≥2 consumers.

## Workflow

1. Confirm ≥2 consumers, stateless, domain-agnostic.
2. Create `utility_<domain>_<role>.py`.
3. Register in `__init__.py`.
4. `python -c "import <module>"`.

## Checklist

- [ ] Only module-level functions — no class.
- [ ] No `self`, no instance state.
- [ ] Pure/deterministic (or I/O justified: domain-agnostic + reusable).
- [ ] No business rules or layer-name knowledge.
- [ ] Used by ≥2 modules.
- [ ] No import from Capabilities, Agent, Surface, Contract.
- [ ] No magic constants (→ `taxonomy_*_constant.py`).
- [ ] `python -c "import <module>"` passes.
