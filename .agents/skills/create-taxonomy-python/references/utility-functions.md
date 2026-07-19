# Utility Functions (`_utility.py`)

## The Ultimate Boundary

A function belongs in `*_utility.py` ONLY if ALL of these are true:

1. Stateless: no `self`, no `cls`, no class field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no randomness, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Multi-consumer reusable: useful for multiple modules/layers.

## Good Utility Example

```python
def match_whole_token(haystack: str, needle: str) -> bool:
    if not needle:
        return False
    import re
    pattern = rf'(?<!\w){re.escape(needle)}(?!\w)'
    return bool(re.search(pattern, haystack))
```

## Bad Utility: Domain Knowledge

```python
def get_target_layer_from_suffix(suffix: str) -> str:
    if suffix == "port":
        return "infrastructure"
    elif suffix == "protocol":
        return "capabilities"
    else:
        return "unknown"
```

This belongs in capabilities as a private helper.

## Bad Utility: Single Consumer Only

```python
def format_import_violation(rule: ImportRuleVO) -> str:
    return f"Import rule violation: {rule.pattern_value}"
```

If only one capability uses it, keep it as a private helper.
