// PURPOSE: NamingVariantAnalyzer — transforms symbol names between naming conventions (snake_case, camelCase, PascalCase, etc.)

use crate::naming_rules::taxonomy_name_vo::SymbolName;
use crate::naming_rules::taxonomy_naming_list_vo::SymbolNameList;
use std::collections::HashSet;

/// Business logic for transforming symbol names into various naming conventions.
pub struct NamingVariantAnalyzer {}

/// A mapping of naming convention variants for a symbol.
pub struct NamingVariantDict {
    pub snake_case: String,
    pub pascal_case: String,
    pub camel_case: String,
    pub screaming_snake: String,
    pub kebab_case: String,
}

impl Default for NamingVariantAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl NamingVariantAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Split a name into lowercase words by camelCase, PascalCase, underscores, hyphens.
    fn split_words(name: &str) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        let mut current = String::new();

        for ch in name.chars() {
            if ch == '_' || ch == '-' || ch == ' ' {
                if !current.is_empty() {
                    words.push(current.to_lowercase());
                    current = String::new();
                }
            } else if ch.is_uppercase() && !current.is_empty() {
                // New word starts on uppercase after lowercase
                let last_lower = current
                    .chars()
                    .last()
                    .map(|c| c.is_lowercase())
                    .unwrap_or(false);
                if last_lower {
                    words.push(current.to_lowercase());
                    current = String::from(ch);
                } else {
                    current.push(ch);
                }
            } else {
                current.push(ch);
            }
        }
        if !current.is_empty() {
            words.push(current.to_lowercase());
        }
        words
    }

    fn capitalize(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + &c.collect::<String>(),
        }
    }

    /// Get all naming convention variants for the given name.
    pub fn get_variant_dict(&self, name: &SymbolName) -> NamingVariantDict {
        let n = name.value.as_ref();
        let words = Self::split_words(n);

        if words.is_empty() {
            return NamingVariantDict {
                snake_case: n.to_string(),
                pascal_case: n.to_string(),
                camel_case: n.to_string(),
                screaming_snake: n.to_uppercase(),
                kebab_case: n.to_string(),
            };
        }

        let snake_case = words.join("_");
        let screaming_snake = snake_case.to_uppercase();
        let kebab_case = words.join("-");
        let pascal_case: String = words.iter().map(|w| Self::capitalize(w)).collect();
        let camel_case = if words.len() > 1 {
            let mut c = words[0].clone();
            for w in &words[1..] {
                c.push_str(&Self::capitalize(w));
            }
            c
        } else {
            words[0].clone()
        };

        NamingVariantDict {
            snake_case,
            pascal_case,
            camel_case,
            screaming_snake,
            kebab_case,
        }
    }

    /// Returns a unique list of all possible naming variants.
    pub fn build_variants(&self, name: &SymbolName) -> SymbolNameList {
        let dict = self.get_variant_dict(name);
        let mut results: HashSet<String> = HashSet::new();
        results.insert(name.value.clone());
        results.insert(dict.snake_case);
        results.insert(dict.camel_case);
        results.insert(dict.pascal_case);
        results.insert(dict.screaming_snake);
        results.insert(dict.kebab_case);

        SymbolNameList {
            values: results
                .into_iter()
                .map(|s| SymbolName { value: s })
                .collect(),
        }
    }
}
