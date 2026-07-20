# Utility Examples (BAD / GOOD)

## Bad: Struct in Utility

```rust
// BAD — utility must not contain struct definitions
pub struct PathNormalizer {
    separator: String,
}

impl PathNormalizer {
    pub fn new(separator: &str) -> Self {
        Self {
            separator: separator.to_string(),
        }
    }

    pub fn normalize(&self, path: &str) -> String {
        path.replace('/', &self.separator)
    }
}
```

## Good: Stateless Free Function

```rust
// GOOD — utility contains only free functions
pub fn normalize_path_separator(path: &str, separator: &str) -> String {
    path.replace('/', separator)
}
```

## Bad: Business Logic in Utility

```rust
// BAD — utility must not know about business rules
pub fn is_valid_import(import: &str) -> bool {
    // This knows about AES architecture — NOT domain-agnostic
    import.starts_with("shared::") || import.starts_with("crate::common::")
}
```

## Good: Generic String Operation

```rust
// GOOD — generic, domain-agnostic operation
pub fn starts_with_module_prefix(path: &str, prefix: &str) -> bool {
    path.starts_with(prefix)
}
```

## Bad: Mutable State

```rust
// BAD — utility must not have side effects or global state
use std::collections::HashMap;

static mut CACHE: HashMap<String, String> = HashMap::new();

pub fn cached_process(input: &str) -> String {
    unsafe {
        // BAD — global state mutation
        if let Some(val) = CACHE.get(input) {
            return val.clone();
        }
        let result = input.to_uppercase();
        CACHE.insert(input.to_string(), result.clone());
        result
    }
}
```

## Good: Pure Function

```rust
// GOOD — pure function, deterministic output
pub fn to_uppercase(input: &str) -> String {
    input.to_uppercase()
}
```

## Bad: Single-Module Dependency

```rust
// BAD — if only one capability uses this, keep as private helper
pub fn check_import_for_aes204(import: &str) -> bool {
    // This knows about AES204 rule — domain-specific
    import.contains("AES204")
}
```

## Good: Reusable Generic Function

```rust
// GOOD — useful for any module that needs prefix checking
pub fn contains_keyword(text: &str, keyword: &str) -> bool {
    text.contains(keyword)
}
```

## I/O Example (Allowed)

```rust
// OK — stateless, domain-agnostic, reusable across modules
use std::path::Path;
use std::fs;

pub fn read_file_content(path: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

pub fn walk_directory(dir: &Path, extensions: &[&str]) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if extensions.contains(&ext) {
                    files.push(path.display().to_string());
                }
            }
        }
    }
    files
}
```
