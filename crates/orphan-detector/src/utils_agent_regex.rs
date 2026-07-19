use regex::Regex;
use std::sync::OnceLock;

pub fn re_impl_generic() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"impl\s*(?:<[^>]+>)?\s+([A-Za-z0-9_]+)\s+for\s+").ok())
        .as_ref()
}

pub fn re_dyn() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?:Box|Arc)<dyn\s+([A-Za-z0-9_]+)>").ok())
        .as_ref()
}

pub fn re_py_class() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"class\s+\w+\(([^)]+)\)").ok())
        .as_ref()
}

pub fn re_ts_implements() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"class\s+\w+\s+implements\s+(\w+)").ok())
        .as_ref()
}

pub fn extract_aggregate_traits(content: &str) -> Vec<String> {
    let mut traits = Vec::new();

    if let Some(re) = re_impl_generic() {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name.contains("Aggregate") || name.ends_with("Aggregate") {
                traits.push(name);
            }
        }
    }

    if let Some(re) = re_dyn() {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name.contains("Aggregate") || name.ends_with("Aggregate") {
                traits.push(name);
            }
        }
    }

    if let Some(re) = re_py_class() {
        for cap in re.captures_iter(content) {
            for part in cap[1].split(',') {
                let name = part.trim().to_string();
                if name.contains("Aggregate") || name.ends_with("Aggregate") {
                    traits.push(name);
                }
            }
        }
    }

    if let Some(re) = re_ts_implements() {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name.contains("Aggregate") || name.ends_with("Aggregate") {
                traits.push(name);
            }
        }
    }

    traits.sort();
    traits.dedup();
    traits
}
