# Utility Functions (`_utility.rs`)

## The Ultimate Boundary

A function belongs in `*_utility.rs` ONLY if ALL of these are true:

1. Stateless: no `&self`, no struct field access.
2. Domain-agnostic: does not know business rules.
3. Multi-consumer reusable: useful for multiple modules/layers.

I/O is allowed in utility functions (e.g., `walk_source_files`, `default_ignored_paths`).

## Good Utility Example

```rust
pub fn match_whole_token(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    let is_ident_char = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    haystack
        .match_indices(needle)
        .any(|(i, _)| {
            let before_ok = i == 0 || !is_ident_char(haystack.as_bytes()[i - 1]);
            let after_ok = i + needle.len() == haystack.len()
                || !is_ident_char(haystack.as_bytes()[i + needle.len()]);
            before_ok && after_ok
        })
}
```

## Bad Utility: Domain Knowledge

```rust
// BAD: knows AES layer mapping rules
pub fn get_target_layer_from_suffix(suffix: &str) -> &'static str {
    match suffix {
        "port" => "infrastructure",
        "protocol" => "capabilities",
        _ => "unknown",
    }
}
```

This belongs in capabilities as a private helper.

## Bad Utility: Single Consumer Only

```rust
// BAD: only used by one checker
pub fn format_import_violation(rule: &ImportRuleVO) -> String {
    format!("Import rule violation: {}", rule.pattern())
}
```

If only one capability uses it, keep it as a private helper.
