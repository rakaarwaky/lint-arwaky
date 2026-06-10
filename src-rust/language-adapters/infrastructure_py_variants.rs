// PURPOSE: PyVariants — Python naming variant builder (snake_case, PascalCase, SCREAMING_SNAKE)

use crate::language_adapters::contract_variant_port::INamingVariantPort;
use crate::shared_common::taxonomy_name_vo::SymbolName;
use crate::shared_common::taxonomy_naming_error::NamingError;
use crate::shared_common::taxonomy_naming_list_vo::SymbolNameList;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_WORDS: Lazy<Result<Regex, NamingError>> = Lazy::new(|| {
    Regex::new(r"[A-Z]{2,}|[A-Z][a-z0-9]*|[a-z0-9]+")
        .map_err(|e| NamingError::new(ErrorMessage::new(format!("Invalid regex pattern: {}", e))))
});

pub struct PythonNamingVariantProvider {}

impl Default for PythonNamingVariantProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl PythonNamingVariantProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl INamingVariantPort for PythonNamingVariantProvider {
    fn get_variant_dict(&self, name: &SymbolName) -> serde_json::Value {
        let name_str = &name.value;
        let re = match RE_WORDS.as_ref() {
            Ok(r) => r,
            Err(_) => {
                return serde_json::json!({"snake_case": name_str, "pascal_case": name_str, "camel_case": name_str, "screaming_snake": name_str.to_uppercase()})
            }
        };
        let words: Vec<String> = re
            .find_iter(name_str)
            .map(|m| m.as_str().to_lowercase())
            .collect();
        if words.is_empty() {
            serde_json::json!({"snake_case": name_str, "pascal_case": name_str, "camel_case": name_str, "screaming_snake": name_str.to_uppercase()})
        } else {
            let snake_case = words.join("_");
            let first = words[0].clone();
            let rest: String = words[1..]
                .iter()
                .map(|w| {
                    let mut c = w.chars();
                    match c.next() {
                        Some(ch) => ch.to_uppercase().to_string() + c.as_str(),
                        None => String::new(),
                    }
                })
                .collect();
            let camel_case = format!("{}{}", first, rest);
            let pascal_case: String = words
                .iter()
                .map(|w| {
                    let mut c = w.chars();
                    match c.next() {
                        Some(ch) => ch.to_uppercase().to_string() + c.as_str(),
                        None => String::new(),
                    }
                })
                .collect();
            let screaming_snake = snake_case.to_uppercase();
            serde_json::json!({"snake_case": snake_case, "camel_case": camel_case, "pascal_case": pascal_case, "screaming_snake": screaming_snake})
        }
    }

    fn build_variants(&self, name: &SymbolName) -> SymbolNameList {
        let name_str = &name.value;
        let rd = self.get_variant_dict(name);
        let sc = rd
            .get("snake_case")
            .and_then(|v| v.as_str())
            .unwrap_or(name_str)
            .to_string();
        let ss = rd
            .get("screaming_snake")
            .and_then(|v| v.as_str())
            .unwrap_or(&name_str.to_uppercase())
            .to_string();
        let cc = rd
            .get("camel_case")
            .and_then(|v| v.as_str())
            .unwrap_or(name_str)
            .to_string();
        let pc = rd
            .get("pascal_case")
            .and_then(|v| v.as_str())
            .unwrap_or(name_str)
            .to_string();
        let kebab = sc.replace('_', "-");
        let mut variants: Vec<String> = vec![name_str.clone(), sc, ss, cc, pc, kebab];
        variants.sort();
        variants.dedup();
        SymbolNameList {
            values: variants.into_iter().map(SymbolName::new).collect(),
        }
    }
}
