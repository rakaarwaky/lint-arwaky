/// ast_py_scanner — Orchestrator for Python AST analysis.
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::LazyLock;

use crate::contract::source_parser_port::ISourceParserPort;
use crate::taxonomy::{
    BooleanVO, ColumnNumber, Count, ErrorMessage, FilePath, ImportInfo, ImportInfoList, LineNumber,
    MetadataVO, PatternList, PrimitiveTypeList, PrimitiveViolation, PrimitiveViolationList,
    ResponseData, SourceParserError, SuccessStatus, SymbolName,
};

static IMPORT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^import\s+(\w+(?:\.\w+)*)").unwrap());
static FROM_IMPORT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^from\s+(\w+(?:\.\w+)*)\s+import\s+(.+)$").unwrap());
static CLASS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^class\s+(\w+)\s*(?:\(([^)]*)\))?:").unwrap());
static DEF_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^def\s+(\w+)\s*\(").unwrap());
static CF_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(if|for|while|try|except|with|async for)\b").unwrap());
static LET_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\w+)\s*=").unwrap());
static WORD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap());
static TYPE_ANNOT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r":\s*(int|str|float|bool|list|dict|tuple|set|bytes|None)\b").unwrap()
});
static RETURN_TYPE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"->\s*(int|str|float|bool|list|dict|tuple|set|bytes|None)\b").unwrap()
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

pub struct ASTPythonParserAdapter;

impl ASTPythonParserAdapter {
    pub fn new() -> Self {
        Self
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

        for (idx_zero, line) in lines.iter().enumerate() {
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
            if let Some(imp_cap) = IMPORT_REGEX.captures(stripped) {
                let module = imp_cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                let alias = module.split('.').last().unwrap_or(&module).to_string();
                data.imported_aliases.insert(alias.clone(), module.clone());
                data.imports_list.push(ImportInfo {
                    line: LineNumber::new((idx_zero + 1) as i64),
                    module,
                    name: None,
                });
            } else if let Some(from_cap) = FROM_IMPORT_REGEX.captures(stripped) {
                let module = from_cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                let symbols_part = from_cap.get(2).map(|m| m.as_str()).unwrap_or("").trim();
                for sym in symbols_part
                    .split(',')
                    .map(|s| s.trim().split(" as ").next().unwrap_or("").trim())
                {
                    if sym.is_empty() || sym.starts_with('(') {
                        continue;
                    }
                    let clean = sym.trim();
                    if !clean.is_empty() {
                        let fullname = format!("{}.{}", module, clean);
                        data.imported_aliases
                            .insert(clean.to_string(), fullname.clone());
                        data.imports_list.push(ImportInfo {
                            line: LineNumber::new((idx_zero + 1) as i64),
                            module: fullname,
                            name: Some(clean.to_string()),
                        });
                    }
                }
                // handle "from X import (" multi-line
                if symbols_part.trim() == "(" {
                    // simple case: single-line from X import (A, B, C)
                    if let Some(close_paren) = symbols_part.rfind(')') {
                        let inner = &symbols_part[1..close_paren];
                        for sym in inner.split(',').map(|s| s.trim()) {
                            let clean = sym.trim();
                            if !clean.is_empty() {
                                let fullname = format!("{}.{}", module, clean);
                                data.imported_aliases
                                    .insert(clean.to_string(), fullname.clone());
                            }
                        }
                    }
                }
            }

            // 2. Class definitions
            if let Some(cls_cap) = CLASS_REGEX.captures(stripped) {
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
            if let Some(fn_cap) = DEF_REGEX.captures(stripped) {
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
            if let Some(let_cap) = LET_REGEX.captures(stripped) {
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
            data.control_flow_count += CF_REGEX.find_iter(stripped).count() as i64;

            // 6. Used symbols
            for cap in WORD_REGEX.find_iter(stripped) {
                let word = cap.as_str();
                if !PY_KEYWORDS.contains(word) && !word.starts_with(|c: char| c.is_numeric()) {
                    data.used.insert(word.to_string());
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
                if let Some(cap) = CLASS_REGEX.captures(stripped) {
                    class_name = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                    in_class = true;
                    class_indent = line.len() - stripped.len();
                    continue;
                }
                if in_class {
                    let indent = line.len() - stripped.len();
                    if indent <= class_indent && !stripped.starts_with('#') && !stripped.is_empty() {
                        if stripped.starts_with("def ") || stripped.starts_with("class ") {
                            break;
                        }
                    }
                    if indent > class_indent && stripped.contains('=') && !stripped.starts_with("def ") {
                        let field_name = stripped.split('=').next().unwrap_or("").trim().to_string();
                        if !field_name.is_empty() && !field_name.starts_with('_') {
                            attrs.entry(class_name.clone())
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
            if let Some(m) = TYPE_ANNOT_RE.find(stripped) {
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
            if let Some(m) = RETURN_TYPE_RE.find(stripped) {
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
            // simple pattern: word: type = or word: type  (on its own line or after comment)
            let attr_re = Regex::new(
                r"\b([a-zA-Z_]\w*)\s*:\s*(int|str|float|bool|list|dict|tuple|set|bytes|None)\b",
            )
            .expect("valid regex");
            for cap in attr_re.captures_iter(stripped) {
                let prim = cap.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
                if prim_set.contains(&prim) {
                    violations.push(PrimitiveViolation {
                        line: LineNumber::new((idx_zero + 1) as i64),
                        column: ColumnNumber::new((cap.get(2).map(|m| m.start()).unwrap_or(0) + 1) as i64),
                        type_name: prim,
                    });
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
