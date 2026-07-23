# Docstring Rules (PEP 257)

## Purpose

Docstrings provide API documentation visible in IDEs, `help()`, and generated docs.

## Rules

1. **Every public module** — one-liner at top of file
2. **Every public class** — one-liner describing purpose
3. **Every public function/method** — describe purpose, parameters, return values, exceptions
4. **Explain "what" and "why"** — not "how" (code shows how)
5. **Use Args section** — document each parameter
6. **Use Returns section** — document return value
7. **Use Raises section** — document exception conditions
8. **Use triple double quotes** — `"""docstring"""`

## Template

```python
"""One-liner describing module purpose."""


class MyClass:
    """One-liner describing class purpose."""

    def my_method(self, param1: str, param2: int) -> bool:
        """One-liner describing method purpose.

        Args:
            param1: Description of param1
            param2: Description of param2

        Returns:
            Description of return value

        Raises:
            ValueError: When error condition occurs
        """
        # ...
```

## Anti-Patterns

- ❌ Missing module docstrings → every file needs one-liner at top
- ❌ Missing parameter documentation → all parameters must be documented
- ❌ Using type: ignore without reason → fix root cause instead
- ❌ Over-documenting obvious code → keep concise and meaningful
- ❌ Explaining "how" instead of "what/why" → code shows how
