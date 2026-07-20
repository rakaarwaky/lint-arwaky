# Helper vs Utility Decision

The boundary is not only about `self`/`cls`.

The real question is:

> Does this function know about specific business/domain rules, or is it just a blind reusable tool?
>
> AND
>
> Is it used only by this class, or by multiple modules?

## Keep as Private Helper in Block 3

Keep the function inside the capabilities file if ANY of these is true:

1. It contains business/domain rules.
2. It accesses `self.field` or instance state.
3. It is tightly coupled to this capability only.
4. It is a factory method such as `create_default()` or `from_config()`.
5. It is stateless but only used by this one class and is domain-specific.

Example:

```python
class ContractRoleChecker:
    def _resolve_scope(self, scope: str) -> tuple[str, list[str]]:
        # Domain-specific parsing logic.
        # Even without `self`, this can remain a private helper
        # if only this checker uses it.
        ...
```

## Extract to Utility (`*_utility.py`)

Extract the function to shared taxonomy utility ONLY if ALL of these are true:

1. Stateless: no `self`, no `cls`, no class field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Reusable: useful for multiple capabilities/utilities/modules.

Example:

```python
# shared/<name-feature>/taxonomy_string_utility.py
def match_whole_token(haystack: str, needle: str) -> bool:
    # generic token matching
    ...
```

## I/O Rule

A function with I/O can be a taxonomy utility if it is stateless, domain-agnostic, and reusable.

```python
# OK in taxonomy utility — stateless, domain-agnostic, reusable
def walk_source_files(dir: str, files: list[str], ignored: list[str]) -> None:
    # I/O is allowed in taxonomy utilities
    ...
```

Rule:

```text
Stateless + I/O + domain-agnostic + reusable = taxonomy utility
Stateless + I/O + domain-specific = utility implementation (infrastructure layer removed)
```

## Decision Tree

```text
Found reusable code in capabilities?
  │
  ├─ Does it know business/domain rules?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need self or class state?
  │   └─ YES → keep as helper/method in Block 3
  │
  └─ Is it stateless, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility (I/O allowed)
```
