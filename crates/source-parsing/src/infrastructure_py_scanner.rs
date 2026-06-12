// PURPOSE: PyScanner — ISourceParserPort for Python import extraction

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::LazyLock;

use shared::code_analysis::taxonomy_import_source_vo::ImportInfo;
use shared::code_analysis::taxonomy_import_source_vo::ImportInfoList;
use shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolation;
use shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList;
use shared::language_adapters::taxonomy_naming_list_vo::PrimitiveTypeList;
use shared::pipeline_jobs::taxonomy_job_vo::ResponseData;
use shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::BooleanVO;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::Count;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_suggestion_vo::MetadataVO;
use shared::source_parsing::contract_parser_port::ISourceParserPort;
use shared::source_parsing::taxonomy_parser_error::SourceParserError;
use shared::source_parsing::taxonomy_path_vo::FilePath;

static IMPORT_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^import\s+(\w+(?:\.\w+)*)(?:\s+as\s+(\w+))?").ok());
static FROM_IMPORT_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^from\s+(\w+(?:\.\w+)*)\s+import\s+(.+)$").ok());
static CLASS_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^class\s+(\w+)\s*(?:\(([^)]*)\))?:").ok());
static DEF_REGEX: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^def\s+(\w+)\s*\(").ok());
static CF_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"\b(if|for|while|try|except|with|async for)\b").ok());
static LET_REGEX: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^(\w+)\s*=").ok());
static WORD_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").ok());
static TYPE_ANNOT_RE: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r":\s*(int|str|float|bool|list|dict|tuple|set|bytes|None)\b").ok());
static RETURN_TYPE_RE: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(r"->\s*(int|str|float|bool|list|dict|tuple|set|bytes|None)\b").ok()
});
static ATTR_ANNOT_RE: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(r"\b([a-zA-Z_]\w*)\s*:\s*(int|str|float|bool|list|dict|tuple|set|bytes|None)\b").ok()
});

static PY_KEYWORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "False",
        "None",
        "True",
        "and",
        "as",
        "assert",
        "async",
        "await",
        "break",
        "class",
        "continue",
        "def",
        "del",
        "elif",
        "else",
        "except",
        "finally",
        "for",
        "from",
        "global",
        "if",
        "import",
        "in",
        "is",
        "lambda",
        "nonlocal",
        "not",
        "or",
        "pass",
        "raise",
        "return",
        "try",
        "while",
        "with",
        "yield",
        "print",
        "len",
        "range",
        "int",
        "str",
        "float",
        "bool",
        "list",
        "dict",
        "tuple",
        "set",
        "bytes",
        "type",
        "super",
        "self",
        "open",
        "isinstance",
        "hasattr",
        "getattr",
        "setattr",
        "map",
        "filter",
        "zip",
        "enumerate",
        "sorted",
        "reversed",
    ]
    .iter()
    .cloned()
    .collect()
});

#[derive(Debug, Default)]
struct ParsedData {
    defined: HashSet<String>,
    used: HashSet<String>,
    exported: HashSet<String>,
    imported_aliases: HashMap<String, String>,
    class_bases: HashMap<String, Vec<String>>,
    imports_list: Vec<ImportInfo>,
    class_definitions: Vec<serde_json::Value>,
    function_definitions: Vec<serde_json::Value>,
    class_methods: HashMap<String, Vec<String>>,
    assignments: Vec<serde_json::Value>,
    control_flow_count: i64,
    is_barrel: bool,
}

pub struct ASTPythonParserAdapter {}

impl Default for ASTPythonParserAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ASTPythonParserAdapter {
    pub fn new() -> Self {
        Self {}
    }

    fn read_and_parse(&self, path: &FilePath) -> Result<ParsedData, SourceParserError> {
        let content = fs::read_to_string(&path.value).map_err(|e| SourceParserError {
            path: path.clone(),
            message: ErrorMessage::new(format!("Failed to read file: {}", e)),
            ..Default::default()
        })?;

        let mut data = ParsedData::default();
        let lines: Vec<&str> = content.lines().collect();
        let mut current_class: Option<String> = None;
        let mut indent_stack: Vec<(usize, Option<String>)> = vec![(0, None)];

        data.is_barrel = path.value.ends_with("__init__.py");

        let mut skip_until = 0;
        for (idx_zero, line) in lines.iter().enumerate() {
            if idx_zero < skip_until {
                continue;
            }
            let stripped = line.trim();
            if stripped.is_empty() || stripped.starts_with('#') {
                continue;
            }

            let indent = line.len() - stripped.len();

            // Track class scope via indentation
            while let Some(&(last_indent, _)) = indent_stack.last() {
                if indent <= last_indent {
                    let (_, cls) = indent_stack.pop().unwrap_or((0, None));
                    if cls.is_some() {
                        current_class = indent_stack.last().and_then(|(_, c)| c.clone());
                    }
                } else {
                    break;
                }
            }

            // 1. Imports
            if let Some(imp_cap) = IMPORT_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                let module = imp_cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                let alias = if let Some(alias_match) = imp_cap.get(2) {
                    alias_match.as_str().to_string()
                } else {
                    module.split('.').next_back().unwrap_or(&module).to_string()
                };
                data.imported_aliases.insert(alias.clone(), module.clone());
                data.imports_list.push(ImportInfo {
                    line: LineNumber::new((idx_zero + 1) as i64),
                    module,
                    name: None,
                });
            } else if let Some(from_cap) = FROM_IMPORT_REGEX
                .as_ref()
                .and_then(|r| r.captures(stripped))
            {
                let module = from_cap
                    .get(1)
                    .map(|m| m.as_str())
                    .unwrap_or("")
                    .to_string();
                let symbols_part = from_cap.get(2).map(|m| m.as_str()).unwrap_or("").trim();

                // Handle parenthesized imports: from X import (A, B, C)
                // single-line or multi-line (opening paren on import line)
                let unwrapped: String = if symbols_part.starts_with('(') {
                    let mut combined = symbols_part.to_string();
                    // Multi-line: closing paren may be on subsequent lines
                    if !combined.contains(')') {
                        let mut j = idx_zero + 1;
                        while j < lines.len() {
                            let cont = lines[j].trim();
                            combined.push(' ');
                            combined.push_str(cont);
                            if cont.contains(')') {
                                skip_until = j + 1;
                                break;
                            }
                            j += 1;
                        }
                    }
                    // Strip parentheses, extract inner content
                    combined
                        .trim_start_matches('(')
                        .trim_end_matches(')')
                        .trim()
                        .to_string()
                } else {
                    symbols_part.to_string()
                };

                for sym in unwrapped
                    .split(',')
                    .map(|s| s.trim())
                {
                    let clean = sym.trim();
                    if clean.is_empty() || clean == "(" || clean == ")" {
                        continue;
                    }
                    if !clean.is_empty() {
                        let (name, alias) = if clean.contains(" as ") {
                            let parts: Vec<&str> = clean.split(" as ").collect();
                            (parts[0].trim().to_string(), parts[1].trim().to_string())
                        } else {
                            (clean.to_string(), clean.to_string())
                        };
                        let fullname = format!("{}.{}", module, name);
                        data.imported_aliases
                            .insert(alias.clone(), fullname.clone());
                        data.imports_list.push(ImportInfo {
                            line: LineNumber::new((idx_zero + 1) as i64),
                            module: fullname,
                            name: Some(name),
                        });
                    }
                }
            }

            // 2. Class definitions
            if let Some(cls_cap) = CLASS_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                let name = cls_cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                data.defined.insert(name.clone());

                let bases_str = cls_cap.get(2).map(|m| m.as_str()).unwrap_or("");
                let bases: Vec<String> = bases_str
                    .split(',')
                    .map(|b| {
                        let clean = b.trim().trim_end_matches(')').trim();
                        clean.split('(').next().unwrap_or(clean).trim().to_string()
                    })
                    .filter(|b| !b.is_empty() && b != "object")
                    .collect();

                if !bases.is_empty() {
                    data.class_bases.insert(name.clone(), bases.clone());
                }

                let resolved_bases: Vec<String> = bases
                    .iter()
                    .map(|b| {
                        data.imported_aliases
                            .get(b)
                            .cloned()
                            .unwrap_or_else(|| b.clone())
                    })
                    .collect();

                data.class_definitions.push(serde_json::json!({
                    "name": name,
                    "line": (idx_zero + 1) as i64,
                    "column": (line.find(&name).unwrap_or(0)) as i64,
                    "is_dead": false,
                    "bases": bases,
                    "resolved_bases": resolved_bases,
                }));

                current_class = Some(name.clone());
                indent_stack.push((indent, current_class.clone()));
            }

            // 3. Function/method definitions
            if let Some(fn_cap) = DEF_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                let name = fn_cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                data.defined.insert(name.clone());

                if let Some(ref cname) = current_class {
                    data.class_methods
                        .entry(cname.clone())
                        .or_insert_with(Vec::new)
                        .push(name);
                } else {
                    data.function_definitions.push(serde_json::json!({
                        "name": name,
                        "line": (idx_zero + 1) as i64,
                        "column": (line.find(&name).unwrap_or(0)) as i64,
                    }));
                }
            }

            // 4. Assignments
            if let Some(let_cap) = LET_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                let name = let_cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                if !PY_KEYWORDS.contains(name.as_str()) {
                    data.assignments.push(serde_json::json!({
                        "name": name,
                        "type": "Assign",
                        "line": (idx_zero + 1) as i64,
                        "column": (line.find(&name).unwrap_or(0)) as i64,
                    }));
                }
            }

            // 5. Control flow
            data.control_flow_count += CF_REGEX
                .as_ref()
                .map_or(0, |r| r.find_iter(stripped).count())
                as i64;

            // 6. Used symbols
            if !stripped.starts_with("import ") && !stripped.starts_with("from ") {
                if let Some(word_re) = WORD_REGEX.as_ref() {
                    for cap in word_re.find_iter(stripped) {
                        let word = cap.as_str();
                        if !PY_KEYWORDS.contains(word) && !word.starts_with(|c: char| c.is_numeric()) {
                            data.used.insert(word.to_string());
                        }
                    }
                }
            }
        }

        Ok(data)
    }
}

impl ISourceParserPort for ASTPythonParserAdapter {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
        let data = self.read_and_parse(path)?;
        Ok(ImportInfoList {
            values: data.imports_list,
        })
    }

    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError> {
        let data = self.read_and_parse(path)?;
        let mut map = HashMap::new();
        map.insert(
            "defined".to_string(),
            serde_json::json!(data.defined.into_iter().collect::<Vec<String>>()),
        );
        map.insert(
            "used".to_string(),
            serde_json::json!(data.used.into_iter().collect::<Vec<String>>()),
        );
        map.insert(
            "exported".to_string(),
            serde_json::json!(data.exported.into_iter().collect::<Vec<String>>()),
        );
        map.insert(
            "aliases".to_string(),
            serde_json::json!(data.imported_aliases),
        );
        map.insert(
            "class_bases".to_string(),
            serde_json::json!(data.class_bases),
        );
        Ok(ResponseData {
            value: Some(serde_json::json!(map)),
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0i64,
            metadata: HashMap::new(),
        })
    }

    fn get_class_attributes(&self, path: &FilePath) -> ResponseData {
        let mut attrs = HashMap::new();
        if let Ok(content) = std::fs::read_to_string(&path.value) {
            let lines: Vec<&str> = content.lines().collect();
            let mut in_class = false;
            let mut class_name = String::new();
            let mut class_indent = 0;
            for line in &lines {
                let stripped = line.trim();
                if stripped.is_empty() || stripped.starts_with('#') {
                    continue;
                }
                if let Some(cap) = CLASS_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                    class_name = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                    in_class = true;
                    class_indent = line.len() - stripped.len();
                    continue;
                }
                if in_class {
                    let indent = line.len() - stripped.len();
                    if indent <= class_indent
                        && !stripped.starts_with('#')
                        && !stripped.is_empty()
                        && (stripped.starts_with("def ") || stripped.starts_with("class "))
                    {
                        break;
                    }
                    if indent > class_indent
                        && stripped.contains('=')
                        && !stripped.starts_with("def ")
                    {
                        let field_name =
                            stripped.split('=').next().unwrap_or("").trim().to_string();
                        if !field_name.is_empty() && !field_name.starts_with('_') {
                            attrs
                                .entry(class_name.clone())
                                .or_insert_with(Vec::new)
                                .push(serde_json::json!({"name": field_name}));
                        }
                    }
                }
            }
        }
        ResponseData {
            value: Some(serde_json::json!(attrs)),
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: HashMap::new(),
        }
    }

    fn has_all_export(&self, path: &FilePath) -> SuccessStatus {
        if !self.is_barrel_file(path).value() {
            return SuccessStatus::new(false);
        }
        if let Ok(content) = fs::read_to_string(&path.value) {
            SuccessStatus::new(content.contains("__all__"))
        } else {
            SuccessStatus::new(false)
        }
    }

    fn find_primitive_violations(
        &self,
        path: &FilePath,
        primitive_types: &PrimitiveTypeList,
    ) -> PrimitiveViolationList {
        let mut violations = Vec::new();
        let content = match fs::read_to_string(&path.value) {
            Ok(c) => c,
            Err(_) => return PrimitiveViolationList { values: violations },
        };

        let prim_set: HashSet<String> = primitive_types
            .values
            .iter()
            .map(|p| p.value.clone())
            .collect();

        for (idx_zero, line) in content.lines().enumerate() {
            let stripped = line.trim();
            if stripped.is_empty()
                || stripped.starts_with('#')
                || stripped.starts_with("import ")
                || stripped.starts_with("from ")
            {
                continue;
            }

            // Check type annotations in function defs: def foo(x: int, y: str) -> bool:
            if let Some(m) = TYPE_ANNOT_RE.as_ref().and_then(|r| r.find(stripped)) {
                let prim = m.as_str().trim_start_matches(": ").to_string();
                if prim_set.contains(&prim) {
                    violations.push(PrimitiveViolation {
                        line: LineNumber::new((idx_zero + 1) as i64),
                        column: ColumnNumber::new((m.start() + 1) as i64),
                        type_name: prim,
                    });
                }
            }

            // Check return type annotations: -> int
            if let Some(m) = RETURN_TYPE_RE.as_ref().and_then(|r| r.find(stripped)) {
                let prim = m.as_str().trim_start_matches("-> ").to_string();
                if prim_set.contains(&prim) {
                    violations.push(PrimitiveViolation {
                        line: LineNumber::new((idx_zero + 1) as i64),
                        column: ColumnNumber::new((m.start() + 1) as i64),
                        type_name: prim,
                    });
                }
            }

            // Check class attribute annotations: x: int = 5
            if let Some(ref attr_re) = *ATTR_ANNOT_RE {
                for cap in attr_re.captures_iter(stripped) {
                    let prim = cap.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
                    if prim_set.contains(&prim) {
                        violations.push(PrimitiveViolation {
                            line: LineNumber::new((idx_zero + 1) as i64),
                            column: ColumnNumber::new(
                                (cap.get(2).map(|m| m.start()).unwrap_or(0) + 1) as i64,
                            ),
                            type_name: prim,
                        });
                    }
                }
            }
        }

        PrimitiveViolationList { values: violations }
    }

    fn find_unused_imports(&self, path: &FilePath) -> ImportInfoList {
        let mut unused = Vec::new();
        let data = match self.read_and_parse(path) {
            Ok(d) => d,
            Err(_) => return ImportInfoList { values: unused },
        };

        for imp in data.imports_list {
            let mod_name = imp.module.clone();
            let mut found_use = data.used.contains(&mod_name) || data.exported.contains(&mod_name);

            if !found_use {
                for (alias, fullname) in &data.imported_aliases {
                    if fullname == &mod_name
                        && (data.used.contains(alias) || data.exported.contains(alias))
                    {
                        found_use = true;
                        break;
                    }
                }
            }

            if !found_use {
                unused.push(imp);
            }
        }

        ImportInfoList { values: unused }
    }

    fn get_class_definitions(&self, path: &FilePath) -> Result<MetadataVO, SourceParserError> {
        let data = self.read_and_parse(path)?;
        let mut map = HashMap::new();
        map.insert(
            "classes".to_string(),
            serde_json::json!(data.class_definitions),
        );
        Ok(MetadataVO::new(map))
    }

    fn get_function_definitions(&self, path: &FilePath) -> MetadataVO {
        if let Ok(data) = self.read_and_parse(path) {
            let mut map = HashMap::new();
            map.insert(
                "functions".to_string(),
                serde_json::json!(data.function_definitions),
            );
            MetadataVO::new(map)
        } else {
            MetadataVO::new(HashMap::new())
        }
    }

    fn is_symbol_exported(&self, path: &FilePath, symbol: &SymbolName) -> SuccessStatus {
        if let Ok(content) = fs::read_to_string(&path.value) {
            for line in content.lines() {
                let stripped = line.trim();
                if stripped.starts_with("__all__") && stripped.contains(&symbol.value) {
                    return SuccessStatus::new(true);
                }
            }
        }
        SuccessStatus::new(false)
    }

    fn get_class_methods(&self, path: &FilePath) -> MetadataVO {
        if let Ok(data) = self.read_and_parse(path) {
            let mut map = HashMap::new();
            map.insert(
                "methods".to_string(),
                serde_json::to_value(&data.class_methods).unwrap_or_default(),
            );
            MetadataVO::new(map)
        } else {
            MetadataVO::new(HashMap::new())
        }
    }

    fn get_class_bases_map(&self, path: &FilePath) -> MetadataVO {
        if let Ok(data) = self.read_and_parse(path) {
            let mut map = HashMap::new();
            map.insert(
                "bases".to_string(),
                serde_json::to_value(&data.class_bases).unwrap_or_default(),
            );
            MetadataVO::new(map)
        } else {
            MetadataVO::new(HashMap::new())
        }
    }

    fn get_assignment_targets(&self, path: &FilePath) -> MetadataVO {
        if let Ok(data) = self.read_and_parse(path) {
            let mut map = HashMap::new();
            map.insert(
                "assignments".to_string(),
                serde_json::json!(data.assignments),
            );
            MetadataVO::new(map)
        } else {
            let mut map = HashMap::new();
            map.insert(
                "assignments".to_string(),
                serde_json::json!(Vec::<serde_json::Value>::new()),
            );
            MetadataVO::new(map)
        }
    }

    fn get_control_flow_count(&self, path: &FilePath) -> Count {
        if let Ok(data) = self.read_and_parse(path) {
            Count::new(data.control_flow_count)
        } else {
            Count::new(0)
        }
    }

    fn is_barrel_file(&self, path: &FilePath) -> BooleanVO {
        BooleanVO::new(path.value.ends_with("__init__.py"))
    }

    fn get_stem(&self, path: &FilePath) -> SymbolName {
        let p = std::path::Path::new(&path.value);
        SymbolName::new(
            p.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string(),
        )
    }

    fn is_entry_point(&self, path: &FilePath) -> BooleanVO {
        BooleanVO::new(path.value.ends_with("main.py") || path.value.ends_with("__main__.py"))
    }

    fn get_supported_extensions(&self) -> PatternList {
        PatternList::new(vec![".py".to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::language_adapters::taxonomy_naming_list_vo::PrimitiveTypeList;
    use shared::taxonomy_name_vo::SymbolName;
    use std::fs;

    #[test]
    fn test_python_scanner_full() {
        let test_path_str = "target/test_python_scanner.py";
        let py_code = r#"
# This is a test Python file
import os
import numpy as np
from math import sin, cos as cosine
from datetime import (
    date,
    time as time_alias,
)

class Animal:
    pass

class Dog(Animal):
    def bark(self):
        x = 5
        if x > 3:
            print("Woof")

def run():
    print(sin(0))

def annotated_func(a: int) -> bool:
    return True
"#;

        fs::create_dir_all("target").unwrap();
        fs::write(test_path_str, py_code).unwrap();

        let path = FilePath::new(test_path_str.to_string()).unwrap();
        let adapter = ASTPythonParserAdapter::new();

        // Test extract_imports
        let imports = adapter.extract_imports(&path).unwrap();
        let modules: Vec<String> = imports.values.iter().map(|i| i.module.clone()).collect();
        assert!(modules.contains(&"os".to_string()));
        assert!(modules.contains(&"numpy".to_string()));
        assert!(modules.contains(&"math.sin".to_string()));
        assert!(modules.contains(&"math.cos".to_string()));
        assert!(modules.contains(&"datetime.date".to_string()));
        assert!(modules.contains(&"datetime.time".to_string()));

        // Check aliases map from read_and_parse
        let parsed = adapter.read_and_parse(&path).unwrap();
        assert_eq!(parsed.imported_aliases.get("os").unwrap(), "os");
        assert_eq!(parsed.imported_aliases.get("np").unwrap(), "numpy");
        assert_eq!(parsed.imported_aliases.get("sin").unwrap(), "math.sin");
        assert_eq!(parsed.imported_aliases.get("cosine").unwrap(), "math.cos");
        assert_eq!(parsed.imported_aliases.get("date").unwrap(), "datetime.date");
        assert_eq!(parsed.imported_aliases.get("time_alias").unwrap(), "datetime.time");

        // Test get_raw_symbols
        let response = adapter.get_raw_symbols(&path).unwrap();
        let val_map = response.value.unwrap();
        let defined = val_map.get("defined").unwrap().as_array().unwrap();
        let defined_strs: Vec<&str> = defined.iter().map(|v| v.as_str().unwrap()).collect();
        assert!(defined_strs.contains(&"Animal"));
        assert!(defined_strs.contains(&"Dog"));
        assert!(defined_strs.contains(&"bark"));
        assert!(defined_strs.contains(&"run"));
        assert!(defined_strs.contains(&"annotated_func"));

        // Test find_unused_imports
        let unused = adapter.find_unused_imports(&path);
        let unused_mods: Vec<String> = unused.values.iter().map(|i| i.module.clone()).collect();
        assert!(unused_mods.contains(&"os".to_string()));
        assert!(unused_mods.contains(&"numpy".to_string()));

        // Test class bases
        let bases_map = adapter.get_class_bases_map(&path);
        let bases = bases_map.value.get("bases").unwrap().as_object().unwrap();
        let dog_bases = bases.get("Dog").unwrap().as_array().unwrap();
        assert_eq!(dog_bases[0].as_str().unwrap(), "Animal");

        // Test control flow count
        assert_eq!(adapter.get_control_flow_count(&path).value, 1);

        // Test find_primitive_violations
        let prim_types = PrimitiveTypeList {
            values: vec![
                SymbolName::new("int".to_string()),
                SymbolName::new("bool".to_string()),
            ],
        };
        let violations = adapter.find_primitive_violations(&path, &prim_types);
        assert_eq!(violations.values.len(), 3);

        // General helpers
        assert!(!adapter.is_barrel_file(&path).value());
        assert_eq!(adapter.get_stem(&path).value, "test_python_scanner");
        assert!(!adapter.is_entry_point(&path).value());
        assert!(adapter.get_supported_extensions().values.contains(&".py".to_string()));

        // Clean up
        let _ = fs::remove_file(test_path_str);
    }
}
