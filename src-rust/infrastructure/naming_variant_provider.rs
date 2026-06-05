/// naming_variant_provider — Python naming variant generator.
use crate::contract::naming_variant_port::INamingVariantPort;
use crate::taxonomy::{NamingError, ResponseData, SymbolName, SymbolNameList};
use regex::Regex;

pub struct PythonNamingVariantProvider;

impl PythonNamingVariantProvider {
    pub fn new() -> Self {
        Self
    }

    pub fn get_variant_dict(&self, name: SymbolName) -> Result<ResponseData, NamingError> {
        let name_str = &name.value;
        let words: Vec<String> = Regex::new(r"[A-Za-z][a-z0-9]*|[A-Z]+(?=[A-Z][a-z0-9]|\b)|[0-9]+")
            .unwrap().find_iter(name_str).map(|m| m.as_ref().to_lowercase()).collect();
        if words.is_empty() {
            return Ok(ResponseData::new(serde_json::json!({"snake_case": name_str, "pascal_case": name_str, "camel_case": name_str, "screaming_snake": name_str.to_uppercase()})));
        }
        let snake_case = words.join("_");
        let first = words[0].clone();
        let rest: String = words[1..].iter().map(|w| { let mut c = w.chars(); match c.next() { Some(ch) => ch.to_uppercase().to_string() + c.as_ref(), None => String::new() } }).collect();
        let camel_case = format!("{}{}", first, rest);
        let pascal_case: String = words.iter().map(|w| { let mut c = w.chars(); match c.next() { Some(ch) => ch.to_uppercase().to_string() + c.as_ref(), None => String::new() } }).collect();
        let screaming_snake = snake_case.to_uppercase();
        Ok(ResponseData::new(serde_json::json!({"snake_case": snake_case, "camel_case": camel_case, "pascal_case": pascal_case, "screaming_snake": screaming_snake})))
    }

    pub fn build_variants(&self, name: SymbolName) -> Result<SymbolNameList, NamingError> {
        let name_str = &name.value;
        let data = self.get_variant_dict(name)?;
        let map = data.value.as_object().cloned().unwrap_or_default();
        let sc = map.get("snake_case").and_then(|v| v.as_ref()).unwrap_or(name_str).to_string();
        let ss = map.get("screaming_snake").and_then(|v| v.as_ref()).unwrap_or(&name_str.to_uppercase()).to_string();
        let cc = map.get("camel_case").and_then(|v| v.as_ref()).unwrap_or(name_str).to_string();
        let pc = map.get("pascal_case").and_then(|v| v.as_ref()).unwrap_or(name_str).to_string();
        let kebab = sc.replace('_', "-");
        let mut variants: Vec<String> = vec![name_str.clone(), sc, ss, cc, pc, kebab];
        variants.sort();
        variants.dedup();
        Ok(SymbolNameList::new(variants.into_iter().map(|v| SymbolName::new(v)).collect()))
    }
}

#[async_trait::async_trait]
impl INamingVariantPort for PythonNamingVariantProvider {
    async fn get_variant_dict(&self, name: &SymbolName) -> Result<ResponseData, NamingError> {
        unimplemented!()
    }
    async fn build_variants(&self, name: &SymbolName) -> Result<Vec<SymbolName>, NamingError> {
        unimplemented!()
    }
}

