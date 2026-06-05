/// javascript_call_tracer — Semantic analysis adapter for JavaScript/TypeScript files.
use crate::contract::ISemanticTracerPort;
use crate::taxonomy::{
    CallChainList, Count, DataFlowList, DirectoryPath, ExitCode, FilePath, LineNumber, MetadataVO,
    ResponseData, ResponseDataList, ScopeRef, SemanticError, StdError, StdOutput, SymbolName,
    SymbolNameList,
};
use async_trait::async_trait;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

pub struct JSCallAdapter;

impl JSCallAdapter {
    pub fn new() -> Self {
        Self
    }

    fn get_variant_dict_raw(name: &str) -> std::collections::HashMap<String, String> {
        let words: Vec<String> = Regex::new(r"[A-Za-z][a-z0-9]*|[A-Z]+(?=[A-Z][a-z0-9]|\b)|[0-9]+")
            .unwrap()
            .find_iter(name)
            .map(|m| m.as_str().to_lowercase())
            .collect();

        if words.is_empty() {
            let mut m = std::collections::HashMap::new();
            m.insert("snake_case".to_string(), name.to_string());
            m.insert("camel_case".to_string(), name.to_string());
            m.insert("pascal_case".to_string(), name.to_string());
            m.insert("screaming_snake".to_string(), name.to_uppercase());
            return m;
        }

        let snake = words.join("_");
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

        let pascal: String = words
            .iter()
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    Some(ch) => ch.to_uppercase().to_string() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect();

        let mut m = std::collections::HashMap::new();
        m.insert("snake_case".to_string(), snake.clone());
        m.insert("camel_case".to_string(), format!("{}{}", first, rest));
        m.insert("pascal_case".to_string(), pascal);
        m.insert("screaming_snake".to_string(), snake.to_uppercase());
        m
    }

    fn build_variants_raw(name: &str) -> Vec<String> {
        let d = Self::get_variant_dict_raw(name);
        let kebab = d.get("snake_case").unwrap().replace("_", "-");

        let mut set = std::collections::HashSet::new();
        set.insert(name.to_string());
        set.insert(d.get("snake_case").unwrap().to_string());
        set.insert(d.get("camel_case").unwrap().to_string());
        set.insert(d.get("pascal_case").unwrap().to_string());
        set.insert(d.get("screaming_snake").unwrap().to_string());
        set.insert(kebab);

        set.into_iter().collect()
    }

    fn find_js_files(root: &Path) -> Vec<PathBuf> {
        let mut js_files = Vec::new();
        let mut dirs = vec![root.to_path_buf()];

        while let Some(dir) = dirs.pop() {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        dirs.push(path);
                    } else if path.is_file() {
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            if matches!(ext, "js" | "jsx" | "ts" | "tsx" | "mjs") {
                                js_files.push(path);
                            }
                        }
                    }
                }
            }
        }
        js_files
    }
}

#[async_trait]
impl ISemanticTracerPort for JSCallAdapter {
    async fn get_enclosing_scope(
        &self,
        _file_path: &FilePath,
        _line: LineNumber,
    ) -> Result<Option<ScopeRef>, SemanticError> {
        Ok(None)
    }

    async fn trace_call_chain(
        &self,
        root_dir: &DirectoryPath,
        target_name: &SymbolName,
    ) -> Result<CallChainList, SemanticError> {
        let mut callers = Vec::new();
        let name = target_name.to_string();
        let root = Path::new(root_dir.as_str());

        let call_pattern = Regex::new(&format!(r"\b{}\s*\(", regex::escape(&name))).unwrap();
        let def_pattern =
            Regex::new(&format!(r"(?:function|class)\s+{}\b", regex::escape(&name))).unwrap();

        let js_files = Self::find_js_files(root);

        for filepath in js_files {
            if let Ok(content) = fs::read_to_string(&filepath) {
                for (i, line) in content.lines().enumerate() {
                    if call_pattern.is_match(line) && !def_pattern.is_match(line) {
                        if let Ok(rel_path) = filepath.strip_prefix(root) {
                            callers.push(format!(
                                "{}:{} -> {}",
                                rel_path.display(),
                                i + 1,
                                line.trim()
                            ));
                        }
                    }
                }
            }
        }

        Ok(CallChainList { values: callers })
    }

    async fn find_flow(
        &self,
        _file_path: &FilePath,
        _var_name: &SymbolName,
        _start_line: LineNumber,
    ) -> DataFlowList {
        DataFlowList { values: Vec::new() }
    }

    async fn get_variant_dict(&self, name: &SymbolName) -> ResponseData {
        let dict = Self::get_variant_dict_raw(&name.to_string());
        let mut map = std::collections::HashMap::new();
        for (k, v) in dict {
            map.insert(k, serde_json::Value::String(v));
        }
        ResponseData::new(
            StdOutput::new(""),
            StdError::new(""),
            ExitCode::new(0),
            MetadataVO::new(map),
        )
    }

    async fn project_wide_rename(
        &self,
        root_dir: &DirectoryPath,
        old_name: &SymbolName,
        new_name: &SymbolName,
    ) -> u32 {
        let root = Path::new(root_dir.as_str());
        let old = old_name.to_string();
        let new = new_name.to_string();

        let pattern = Regex::new(&format!(
            r"(?x)
            (
                `(?:\\.|[^`\\])*`             |
                \x22(?:\\.|[^\\x22\\])*\x22   |
                '(?:\\.|[^'\\])*'             |
                //[^\n]*                      |
                /\*(?:.|\n)*?\*/
            )
            |
            \b({})\b
            ",
            regex::escape(&old)
        ))
        .unwrap();

        let js_files = Self::find_js_files(root);
        let mut modified_count = 0;

        for filepath in js_files {
            if let Ok(source) = fs::read_to_string(&filepath) {
                if source.contains(&old) {
                    let new_source = pattern.replace_all(&source, |caps: &regex::Captures| {
                        if let Some(m) = caps.get(1) {
                            m.as_str().to_string()
                        } else {
                            new.clone()
                        }
                    });

                    if new_source != source {
                        if fs::write(&filepath, new_source.as_ref()).is_ok() {
                            modified_count += 1;
                        }
                    }
                }
            }
        }

        modified_count
    }

    async fn get_symbol_locations(
        &self,
        _file_path: &FilePath,
        _symbol: &SymbolName,
    ) -> Vec<ResponseData> {
        Vec::new()
    }

    async fn build_variants(&self, name: &SymbolName) -> Vec<SymbolName> {
        Self::build_variants_raw(&name.to_string())
            .into_iter()
            .filter_map(|s| SymbolName::try_from(s).ok())
            .collect()
    }
}
