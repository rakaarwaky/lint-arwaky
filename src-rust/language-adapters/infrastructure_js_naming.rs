/// javascript_naming_provider — Naming variants for JavaScript/TypeScript symbols.
use crate::naming_rules::contract_provider_port::INamingProviderPort;
use /* UNKNOWN: ErrorMessage */ crate::shared_common::taxonomy_common_error::ErrorMessage;
use /* UNKNOWN: NameVariants */ crate::naming_rules::taxonomy_symbol_vo::NameVariants;
use crate::naming_rules::taxonomy_provider_error::NamingError;
use /* UNKNOWN: SymbolName */ crate::naming_rules::taxonomy_symbol_vo::SymbolName;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_WORDS: Lazy<Result<Regex, NamingError>> = Lazy::new(|| {
    Regex::new(r"[A-Za-z][a-z0-9]*|[A-Z]+(?=[A-Z][a-z0-9]|\b)|[0-9]+")
        .map_err(|e| NamingError::new(ErrorMessage::new(format!("Invalid regex pattern: {}", e))))
});

pub struct JavascriptNamingProvider {}

impl JavascriptNamingProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl INamingProviderPort for JavascriptNamingProvider {
    fn get_variants(&self, name: &SymbolName) -> NameVariants {
        let name_str = &name.value;
        let re = match RE_WORDS.as_ref() {
            Ok(r) => r,
            Err(_) => return NameVariants::new(vec![SymbolName::new(name_str.clone())]),
        };
        let words: Vec<String> = re
            .find_iter(name_str)
            .map(|m| m.as_str().to_lowercase())
            .collect();
        if words.is_empty() {
            return NameVariants::new(vec![SymbolName::new(name_str.clone())]);
        }
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
        let kebab = snake_case.replace('_', "-");
        let mut variants = vec![
            name_str.clone(),
            snake_case,
            camel_case,
            pascal_case,
            screaming_snake,
            kebab,
        ];
        variants.sort();
        variants.dedup();
        NameVariants::new(variants.into_iter().map(SymbolName::new).collect())
    }
}
