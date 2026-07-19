use regex::Regex;
use std::sync::OnceLock;

/// Regex for `#[path = "..."] pub mod` declarations
pub fn pub_mod_path_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r#"\[path\s*=\s*"([^"]+)"\]\s*pub\s+mod"#).ok())
        .as_ref()
}

/// Regex for `mod name;` declarations (plain mod without path attribute)
pub fn plain_mod_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^\s*mod\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*;").ok())
        .as_ref()
}

/// Regex for `use`, `import`, `from ... import` statements
pub fn import_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?:use\s+([a-zA-Z_][a-zA-Z0-9_:]*)|from\s+([a-zA-Z_.]+)\s+import|import\s+([a-zA-Z_][a-zA-Z0-9_.]*))",
        )
        .ok()
    })
    .as_ref()
}

/// Regex for Python class inheritance: `class Foo(Bar, Baz):`
pub fn inh_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"class\s+\w+\(([^)]+)\)").ok())
        .as_ref()
}
