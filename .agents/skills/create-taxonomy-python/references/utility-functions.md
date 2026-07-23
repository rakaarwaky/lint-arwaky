# Utility Functions (`_utility.py`)

## The Ultimate Boundary

A function belongs in `*_utility.py` ONLY if ALL of these are true:

1. Stateless: no `self`, no `cls`, no class field access.
2. Domain-agnostic: does not know business rules.
3. Multi-consumer reusable: useful for multiple modules/layers.

I/O is allowed in utility functions (e.g., `walk_source_files`, `default_ignored_paths`).

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
        return "capabilities"
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
