/// naming_variant_provider — Python naming variant generator.
use crate::contract::naming_variant_port::INamingVariantPort;
use crate::taxonomy::{SymbolName, SymbolNameList};
use regex::Regex;

pub struct PythonNamingVariantProvider;

impl PythonNamingVariantProvider {
    pub fn new() -> Self {
        Self
    }
}

impl INamingVariantPort for PythonNamingVariantProvider {
    fn get_variant_dict(&self, name: &SymbolName) -> serde_json::Value {
        let name_str = &name.value;
        let words: Vec<String> = Regex::new(r"[A-Za-z][a-z0-9]*|[A-Z]+(?=[A-Z][a-z0-9]|\b)|[0-9]+")
            .unwrap().find_iter(name_str).map(|m| m.as_str().to_lowercase()).collect();
        if words.is_empty() {
            return serde_json::json!({"snake_case": name_str, "pascal_case": name_str, "camel_case": name_str, "screaming_snake": name_str.to_uppercase()});
        }
        let snake_case = words.join("_");
        let first = words[0].clone();
        let rest: String = words[1..].iter().map(|w| {
            let mut c = w.chars();
            match c.next() { Some(ch) => ch.to_uppercase().to_string() + c.as_str(), None => String::new() }
        }).collect();
        let camel_case = format!("{}{}", first, rest);
        let pascal_case: String = words.iter().map(|w| {
            let mut c = w.chars();
            match c.next() { Some(ch) => ch.to_uppercase().to_string() + c.as_str(), None => String::new() }
        }).collect();
        let screaming_snake = snake_case.to_uppercase();
        serde_json::json!({"snake_case": snake_case, "camel_case": camel_case, "pascal_case": pascal_case, "screaming_snake": screaming_snake})
    }

    fn build_variants(&self, name: &SymbolName) -> SymbolNameList {
        let name_str = &name.value;
        let map = self.get_variant_dict(name);
        let sc = map.get("snake_case").and_then(|v| v.as_str()).unwrap_or(name_str).to_string();
        let ss = map.get("screaming_snake").and_then(|v| v.as_str()).unwrap_or(&name_str.to_uppercase()).to_string();
        let cc = map.get("camel_case").and_then(|v| v.as_str()).unwrap_or(name_str).to_string();
        let pc = map.get("pascal_case").and_then(|v| v.as_str()).unwrap_or(name_str).to_string();
        let kebab = sc.replace('_', "-");
        let mut variants: Vec<String> = vec![name_str.clone(), sc, ss, cc, pc, kebab];
        variants.sort();
        variants.dedup();
        SymbolNameList { values: variants.into_iter().map(SymbolName::new).collect() }
    }
}
