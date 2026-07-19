use regex::Regex;
use std::sync::OnceLock;

pub fn re_contract_rust() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

pub fn re_contract_py() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?:class\s+([A-Za-z0-9_]+)\s*\([^)]*ABC[^)]*\)|class\s+([A-Za-z0-9_]+)\s*\([^)]*Protocol[^)]*\))").ok()).as_ref()
}

pub fn re_contract_py_fallback() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"class\s+([A-Za-z0-9_]+)\s*[\(:]").ok())
        .as_ref()
}

pub fn re_ts_interface_export() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"export\s+interface\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

pub fn re_interface() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"interface\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

pub fn word_boundary_re(trait_name: &str) -> Regex {
    let pattern = format!(r"\b{}\b", regex::escape(trait_name));
    match Regex::new(&pattern) {
        Ok(re) => re,
        Err(_) => never_match_regex(),
    }
}

fn never_match_regex() -> Regex {
    match Regex::new("") {
        Ok(re) => re,
        Err(_) => std::process::abort(),
    }
}
