use crate::contract::source_parser_port::ISourceParserPort;
use crate::taxonomy::{
    BooleanVO, Cause, ColumnNumber, Count, ErrorCode, ErrorMessage, FilePath, ImportInfo,
    ImportInfoList, LineNumber, MetadataVO, ModuleName, PatternList, PrimitiveTypeList,
    PrimitiveTypeName, PrimitiveViolation, PrimitiveViolationList, ResponseData, SourceParserError,
    SuccessStatus, SymbolName,
};
use std::collections::{HashMap, HashSet};
use std::fs;
use regex::Regex;
use std::sync::LazyLock;

// ── Rust Parser Adapter ──────────────────────────────────────────────────────

static USE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:pub\s+)?use\s+([^;]+);").unwrap());
static STRUCT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:pub\s+)?(?:pub\s*\([^)]*\)\s+)?(?:struct|enum|trait)\s+([a-zA-Z0-9_]+)").unwrap());
static FN_REGEX_RUST: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:pub\s+)?(?:async\s+)?fn\s+([a-zA-Z0-9_]+)").unwrap());
static IMPL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^impl\s+(?:([a-zA-Z0-9_:]+)\s+for\s+)?([a-zA-Z0-9_]+)").unwrap());
static CF_REGEX_RUST: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(if|for|while|match|loop)\b").unwrap());
static LET_REGEX_RUST: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^let\s+(?:mut\s+)?([a-zA-Z0-9_]+)").unwrap());
static WORD_REGEX_RUST: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap());

static PUB_STRUCT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(struct|enum|trait|fn|const)\s+([a-zA-Z0-9_]+)").unwrap());
static PUB_MOD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bmod\s+([a-zA-Z0-9_]+)").unwrap());
static PUB_USE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\buse\s+(?:.*::)?([a-zA-Z0-9_]+)\s*(?:::\{|;|$)").unwrap());
static PUB_USE_GROUP: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\buse\s+.*::\{([^}]+)\}").unwrap());
static TYPE_DECL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(struct|enum|trait|fn|impl|pub)\b").unwrap());

struct RustParsedData {
    defined: Vec<String>,
    used: Vec<String>,
    exported: Vec<String>,
    aliases: HashMap<String, String>,
    class_bases: HashMap<String, Vec<String>>,
    imports_list: Vec<ImportInfo>,
    class_definitions: Vec<serde_json::Value>,
    function_definitions: Vec<serde_json::Value>,
    class_methods: HashMap<String, Vec<String>>,
    assignments: Vec<serde_json::Value>,
    control_flow_count: i64,
}

pub struct ASTRustParserAdapter;

impl ASTRustParserAdapter {
    pub fn new() -> Self {
        Self
    }

    fn read_and_parse(&self, path: &FilePath) -> Result<RustParsedData, SourceParserError> {
        let content = fs::read_to_string(&path.value).map_err(|e| SourceParserError {
            path: path.clone(),
            message: ErrorMessage::new(format!("Failed to read file: {}", e)),
            error_code: ErrorCode::new("FILE_READ_ERROR"),
            cause: Cause::new(e.to_string()),
        })?;

        let mut defined = HashSet::new();
        let mut used = HashSet::new();
        let mut exported = HashSet::new();
        let mut imported_aliases = HashMap::new();
        let mut class_bases = HashMap::new();
        let mut imports_list = Vec::new();
        let mut class_defs = Vec::new();
        let mut func_defs = Vec::new();
        let mut class_methods: HashMap<String, Vec<String>> = HashMap::new();
        let mut assignments = Vec::new();
        let mut control_flow_count = 0;

        let lines: Vec<&str> = content.lines().collect();
        let mut current_impl: Option<String> = None;
        let mut brace_count = 0;

        for (idx_zero, line) in lines.iter().enumerate() {
            let idx = (idx_zero + 1) as i64;
            let stripped = line.trim();
            if stripped.is_empty() || stripped.starts_with("//") || stripped.starts_with("/*") || stripped.starts_with("*") {
                continue;
            }

            let open_braces = stripped.matches('{').count() as i32;
            let close_braces = stripped.matches('}').count() as i32;

            if let Some(impl_cap) = IMPL_REGEX.captures(stripped) {
                let trait_name = impl_cap.get(1).map(|m| m.as_str());
                let struct_name = impl_cap.get(2).unwrap().as_str().to_string();
                
                if let Some(t_name) = trait_name {
                    let clean_trait = t_name.split("::").last().unwrap_or(t_name).to_string();
                    class_bases
                        .entry(struct_name.clone())
                        .or_insert_with(Vec::new)
                        .push(clean_trait);
                }
                
                current_impl = Some(struct_name);
                brace_count = 0;
            }

            brace_count += open_braces - close_braces;
            if brace_count < 0 {
                brace_count = 0;
                current_impl = None;
            }

            if let Some(use_cap) = USE_REGEX.captures(stripped) {
                let raw_path = use_cap.get(1).unwrap().as_str().trim();
                let mut clean_path = raw_path;
                for prefix in &["crate::", "self::", "super::"] {
                    if clean_path.starts_with(prefix) {
                        clean_path = &clean_path[prefix.len()..];
                    }
                }

                let expanded = if clean_path.contains("::{") {
                    let parts: Vec<&str> = clean_path.split("::{").collect();
                    let prefix = parts[0];
                    let sub_parts: Vec<&str> = parts[1].trim_end_matches('}').split(',').collect();
                    sub_parts
                        .iter()
                        .map(|p| format!("{}::{}", prefix, p.trim()))
                        .filter(|p| !p.is_empty())
                        .collect::<Vec<String>>()
                } else {
                    vec![clean_path.to_string()]
                };

                for item in expanded {
                    let dotted = item.replace("::", ".");
                    let alias = dotted.split('.').last().unwrap_or(&dotted).to_string();
                    if alias != "*" {
                        imported_aliases.insert(alias, dotted.clone());
                        imports_list.push(ImportInfo {
                            line: LineNumber::new(idx),
                            module: ModuleName::new(dotted),
                            name: SymbolName::new(""),
                        });
                    }
                }
            }

            if let Some(struct_cap) = STRUCT_REGEX.captures(stripped) {
                let name = struct_cap.get(1).unwrap().as_str().to_string();
                defined.insert(name.clone());
                
                let mut is_dead = stripped.contains(';') || stripped.contains("{}");
                if !is_dead && (idx_zero + 1) < lines.len() {
                    let next_line = lines[idx_zero + 1].trim();
                    if next_line == "}" || next_line == ";" {
                        is_dead = true;
                    }
                }

                let col_pos = line.find(&name).unwrap_or(0) as i64;
                class_defs.push(serde_json::json!({
                    "name": name,
                    "line": idx,
                    "column": col_pos,
                    "is_dead": is_dead,
                    "bases": Vec::<String>::new(),
                    "resolved_bases": Vec::<String>::new()
                }));
            }

            if let Some(fn_cap) = FN_REGEX_RUST.captures(stripped) {
                let name = fn_cap.get(1).unwrap().as_str().to_string();
                defined.insert(name.clone());
                
                if let Some(ref cimpl) = current_impl {
                    class_methods
                        .entry(cimpl.clone())
                        .or_insert_with(Vec::new)
                        .push(name);
                } else {
                    let col_pos = line.find(&name).unwrap_or(0) as i64;
                    func_defs.push(serde_json::json!({
                        "name": name,
                        "line": idx,
                        "column": col_pos
                    }));
                }
            }

            if stripped.starts_with("let ") {
                if let Some(let_cap) = LET_REGEX_RUST.captures(stripped) {
                    let name = let_cap.get(1).unwrap().as_str().to_string();
                    let col_pos = line.find(&name).unwrap_or(0) as i64;
                    assignments.push(serde_json::json!({
                        "name": name,
                        "type": "Assign",
                        "line": idx,
                        "column": col_pos
                    }));
                }
            }

            control_flow_count += CF_REGEX_RUST.find_iter(stripped).count() as i64;

            for word_match in WORD_REGEX_RUST.find_iter(stripped) {
                used.insert(word_match.as_str().to_string());
            }

            if stripped.starts_with("pub ") {
                if let Some(cap) = PUB_STRUCT_REGEX.captures(stripped) {
                    exported.insert(cap.get(2).unwrap().as_str().to_string());
                }
                if let Some(cap) = PUB_MOD_REGEX.captures(stripped) {
                    exported.insert(cap.get(1).unwrap().as_str().to_string());
                }
                if let Some(cap) = PUB_USE_REGEX.captures(stripped) {
                    exported.insert(cap.get(1).unwrap().as_str().to_string());
                }
                if let Some(cap) = PUB_USE_GROUP.captures(stripped) {
                    for name in cap.get(1).unwrap().as_str().split(',') {
                        let clean = name.trim();
                        if !clean.is_empty() {
                            exported.insert(clean.to_string());
                        }
                    }
                }
            }
        }

        for cdef in &mut class_defs {
            if let Some(obj) = cdef.as_object_mut() {
                if let Some(cname_val) = obj.get("name").and_then(|v| v.as_str()) {
                    let cname = cname_val.to_string();
                    if let Some(bases) = class_bases.get(&cname) {
                        obj.insert("bases".to_string(), serde_json::json!(bases));
                        let resolved: Vec<String> = bases
                            .iter()
                            .map(|b| imported_aliases.get(b).cloned().unwrap_or_else(|| b.clone()))
                            .collect();
                        obj.insert("resolved_bases".to_string(), serde_json::json!(resolved));
                    }
                }
            }
        }

        Ok(RustParsedData {
            defined: defined.into_iter().collect(),
            used: used.into_iter().collect(),
            exported: exported.into_iter().collect(),
            aliases: imported_aliases,
            class_bases,
            imports_list,
            class_definitions: class_defs,
            function_definitions: func_defs,
            class_methods,
            assignments,
            control_flow_count,
        })
    }
}

impl ISourceParserPort for ASTRustParserAdapter {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
        let data = self.read_and_parse(path)?;
        Ok(ImportInfoList { values: data.imports_list })
    }

    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError> {
        let data = self.read_and_parse(path)?;
        let mut map = HashMap::new();
        map.insert("defined".to_string(), serde_json::json!(data.defined));
        map.insert("used".to_string(), serde_json::json!(data.used));
        map.insert("exported".to_string(), serde_json::json!(data.exported));
        map.insert("aliases".to_string(), serde_json::json!(data.aliases));
        map.insert("class_bases".to_string(), serde_json::json!(data.class_bases));
        
        Ok(ResponseData {
            value: serde_json::json!(map),
            stdout: crate::taxonomy::StdOutput::new(""),
            stderr: crate::taxonomy::StdError::new(""),
            returncode: crate::taxonomy::ExitCode::new(0),
            metadata: crate::taxonomy::MetadataVO::new(HashMap::new()),
        })
    }

    fn get_class_attributes(&self, _path: &FilePath) -> ResponseData {
        ResponseData {
            value: serde_json::json!(HashMap::<String, serde_json::Value>::new()),
            stdout: crate::taxonomy::StdOutput::new(""),
            stderr: crate::taxonomy::StdError::new(""),
            returncode: crate::taxonomy::ExitCode::new(0),
            metadata: crate::taxonomy::MetadataVO::new(HashMap::new()),
        }
    }

    fn has_all_export(&self, path: &FilePath) -> SuccessStatus {
        if !self.is_barrel_file(path) {
            return SuccessStatus {
                value: BooleanVO::new(false),
            };
        }
        if let Ok(data) = self.read_and_parse(path) {
            SuccessStatus {
                value: BooleanVO::new(!data.exported.is_empty()),
            }
        } else {
            SuccessStatus {
                value: BooleanVO::new(false),
            }
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
            Err(_) => return PrimitiveViolationList { values: Vec::new() },
        };

        let prim_keywords: Vec<String> = primitive_types
            .values
            .iter()
            .map(|s| s.value.clone())
            .collect();

        for (idx_zero, line) in content.lines().enumerate() {
            let idx = (idx_zero + 1) as i64;
            let stripped = line.trim();
            if stripped.starts_with("use ") || stripped.starts_with("//") || stripped.starts_with("/*") {
                continue;
            }

            if TYPE_DECL_REGEX.is_match(stripped) {
                for prim in &prim_keywords {
                    let prim_regex = Regex::new(&format!(r"\b{}\b", prim)).unwrap();
                    if let Some(m) = prim_regex.find(stripped) {
                        let col = (line.find(m.as_str()).unwrap_or(0) + 1) as i64;
                        violations.push(PrimitiveViolation {
                            line: LineNumber::new(idx),
                            column: ColumnNumber::new(col),
                            type_name: PrimitiveTypeName::new(prim.clone()),
                        });
                    }
                }
            }
        }

        PrimitiveViolationList { values: violations }
    }

    fn find_unused_imports(&self, path: &FilePath) -> ImportInfoList {
        let data = match self.read_and_parse(path) {
            Ok(d) => d,
            Err(_) => return ImportInfoList { values: Vec::new() },
        };

        let used_set: HashSet<String> = data.used.into_iter().collect();
        let exported_set: HashSet<String> = data.exported.into_iter().collect();

        let mut unused = Vec::new();
        for imp in data.imports_list {
            let mod_name = imp.module.value.clone();
            let mut is_used = used_set.contains(&mod_name) || exported_set.contains(&mod_name);
            
            if !is_used {
                for (alias, fullname) in &data.aliases {
                    if fullname == &mod_name && (used_set.contains(alias) || exported_set.contains(alias)) {
                        is_used = true;
                        break;
                    }
                }
            }

            if !is_used {
                unused.push(imp);
            }
        }

        ImportInfoList { values: unused }
    }

    fn get_class_definitions(&self, path: &FilePath) -> Result<MetadataVO, SourceParserError> {
        let data = self.read_and_parse(path)?;
        let mut map = HashMap::new();
        map.insert("classes".to_string(), serde_json::json!(data.class_definitions));
        Ok(MetadataVO { value: map })
    }

    fn get_function_definitions(&self, path: &FilePath) -> MetadataVO {
        let mut map = HashMap::new();
        if let Ok(data) = self.read_and_parse(path) {
            map.insert("functions".to_string(), serde_json::json!(data.function_definitions));
        } else {
            map.insert("functions".to_string(), serde_json::json!(Vec::<serde_json::Value>::new()));
        }
        MetadataVO { value: map }
    }

    fn is_symbol_exported(&self, path: &FilePath, symbol: &SymbolName) -> SuccessStatus {
        if let Ok(data) = self.read_and_parse(path) {
            SuccessStatus {
                value: BooleanVO::new(data.exported.contains(&symbol.value)),
            }
        } else {
            SuccessStatus {
                value: BooleanVO::new(false),
            }
        }
    }

    fn get_class_methods(&self, path: &FilePath) -> MetadataVO {
        let mut map = HashMap::new();
        if let Ok(data) = self.read_and_parse(path) {
            for (cname, methods) in data.class_methods {
                map.insert(cname, serde_json::json!(methods));
            }
        }
        MetadataVO { value: map }
    }

    fn get_class_bases_map(&self, path: &FilePath) -> MetadataVO {
        let mut map = HashMap::new();
        if let Ok(data) = self.read_and_parse(path) {
            for (cname, bases) in data.class_bases {
                map.insert(cname, serde_json::json!(bases));
            }
        }
        MetadataVO { value: map }
    }

    fn get_assignment_targets(&self, path: &FilePath) -> MetadataVO {
        let mut map = HashMap::new();
        if let Ok(data) = self.read_and_parse(path) {
            map.insert("assignments".to_string(), serde_json::json!(data.assignments));
        } else {
            map.insert("assignments".to_string(), serde_json::json!(Vec::<serde_json::Value>::new()));
        }
        MetadataVO { value: map }
    }

    fn get_control_flow_count(&self, path: &FilePath) -> Count {
        if let Ok(data) = self.read_and_parse(path) {
            Count::new(data.control_flow_count)
        } else {
            Count::new(0)
        }
    }

    fn is_barrel_file(&self, path: &FilePath) -> bool {
        let path_str = path.value.replace('\\', "/");
        path_str.ends_with("/mod.rs") || path_str.ends_with("/lib.rs")
    }

    fn get_stem(&self, path: &FilePath) -> SymbolName {
        let path_str = path.value.replace('\\', "/");
        let basename = path_str.split('/').last().unwrap_or(&path.value);
        SymbolName::new(basename.replace(".rs", ""))
    }

    fn is_entry_point(&self, path: &FilePath) -> bool {
        let path_str = path.value.replace('\\', "/");
        let basename = path_str.split('/').last().unwrap_or(&path.value);
        basename == "main.rs" || basename == "lib.rs" || basename == "mod.rs"
    }

    fn get_supported_extensions(&self) -> PatternList {
        PatternList::new(".rs")
    }
}

// ── JS Parser Adapter ────────────────────────────────────────────────────────

static IMPORT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^import\s+(.+?)\s+from\s+'([^']+)'").unwrap());
static IMPORT_DOUBLE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^import\s+(.+?)\s+from\s+"([^"]+)""#).unwrap());
static REQUIRE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^(?:const|let|var)\s+(\w+)\s*=\s*require\((?:'([^']+)'|"([^"]+)")\)"#).unwrap()
});
static CLASS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^class\s+(\w+)(?:\s+extends\s+(\w+))?").unwrap());
static FN_REGEX_JS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:async\s+)?function\s+(\w+)").unwrap());
static CF_REGEX_JS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(if|for|while|switch|catch)\b").unwrap());
static LET_REGEX_JS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:const|let|var)\s+(\w+)\s*=").unwrap());
static METHOD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:async\s+)?(\w+)\s*\([^)]*\)\s*\{").unwrap());
static WORD_REGEX_JS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap());

struct JSParsedData {
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
}

pub struct ASTJSParserAdapter;

impl ASTJSParserAdapter {
    pub fn new() -> Self {
        Self
    }

    fn read_and_parse(&self, path: &FilePath) -> Result<JSParsedData, SourceParserError> {
        let content = fs::read_to_string(&path.value).map_err(|e| SourceParserError {
            path: path.clone(),
            message: ErrorMessage::new(format!("Failed to read file: {}", e)),
            error_code: ErrorCode::new("FILE_READ_ERROR"),
            cause: Cause::new(e.to_string()),
        })?;

        let mut data = JSParsedData {
            defined: HashSet::new(),
            used: HashSet::new(),
            exported: HashSet::new(),
            imported_aliases: HashMap::new(),
            class_bases: HashMap::new(),
            imports_list: Vec::new(),
            class_definitions: Vec::new(),
            function_definitions: Vec::new(),
            class_methods: HashMap::new(),
            assignments: Vec::new(),
            control_flow_count: 0,
        };
        let lines: Vec<&str> = content.lines().collect();

        let mut in_comment = false;
        let mut current_class: Option<String> = None;
        let mut brace_count = 0;

        let js_keywords: HashSet<&str> = [
            "break", "case", "catch", "class", "const", "continue", "debugger", "default",
            "delete", "do", "else", "export", "extends", "finally", "for", "function", "if",
            "import", "in", "instanceof", "new", "return", "super", "switch", "this", "throw",
            "try", "typeof", "var", "void", "while", "with", "yield", "let", "static", "async",
            "await", "from",
        ]
        .iter()
        .cloned()
        .collect();

        for (idx_zero, line) in lines.iter().enumerate() {
            let idx = (idx_zero + 1) as i64;
            let original_stripped = line.trim();

            if in_comment {
                if original_stripped.contains("*/") {
                    in_comment = false;
                }
                continue;
            }

            if original_stripped.starts_with("/*") {
                if !original_stripped.contains("*/") {
                    in_comment = true;
                }
                continue;
            }

            if original_stripped.is_empty()
                || original_stripped.starts_with("//")
                || original_stripped.starts_with('*')
            {
                continue;
            }

            let code_part = match line.find("//") {
                Some(pos) => &line[..pos],
                None => line,
            };

            let stripped = code_part.trim();
            if stripped.is_empty() {
                continue;
            }

            let open_braces = stripped.matches('{').count() as i32;
            let close_braces = stripped.matches('}').count() as i32;

            let mut class_match_found = false;
            if let Some(class_cap) = CLASS_REGEX.captures(stripped) {
                let name = class_cap.get(1).unwrap().as_str().to_string();
                data.defined.insert(name.clone());

                let base = class_cap.get(2).map(|m| m.as_str().to_string());
                let resolved_base = base
                    .as_ref()
                    .map(|b| data.imported_aliases.get(b).cloned().unwrap_or_else(|| b.clone()));

                if let Some(ref b) = base {
                    data.class_bases.insert(name.clone(), vec![b.clone()]);
                }

                let col_pos = line.find(&name).unwrap_or(0) as i64;
                data.class_definitions.push(serde_json::json!({
                    "name": name.clone(),
                    "line": idx,
                    "column": col_pos,
                    "is_dead": false,
                    "bases": base.clone().map(|b| vec![b]).unwrap_or_default(),
                    "resolved_bases": resolved_base.clone().map(|rb| vec![rb]).unwrap_or_default(),
                }));

                current_class = Some(name);
                brace_count = 0;
                class_match_found = true;
            }

            brace_count += open_braces - close_braces;
            if brace_count < 0 {
                brace_count = 0;
                current_class = None;
            }

            if let Some(ref cname) = current_class {
                if !class_match_found {
                    if let Some(m_cap) = METHOD_REGEX.captures(stripped) {
                        let mname = m_cap.get(1).unwrap().as_str().to_string();
                        if !["if", "for", "while", "switch"].contains(&mname.as_str()) {
                            data.class_methods
                                .entry(cname.clone())
                                .or_insert_with(Vec::new)
                                .push(mname);
                        }
                    }
                }
            }

            let mut import_found = false;
            let mut raw_imports = "";
            let mut module_path = "";

            if let Some(imp_cap) = IMPORT_REGEX.captures(stripped) {
                raw_imports = imp_cap.get(1).unwrap().as_str().trim();
                module_path = imp_cap.get(2).unwrap().as_str().trim();
                import_found = true;
            } else if let Some(imp_cap) = IMPORT_DOUBLE_REGEX.captures(stripped) {
                raw_imports = imp_cap.get(1).unwrap().as_str().trim();
                module_path = imp_cap.get(2).unwrap().as_str().trim();
                import_found = true;
            }

            if import_found {
                let mod_path = module_path
                    .replace('/', ".")
                    .trim_start_matches('.')
                    .to_string();
                if raw_imports.contains('{') {
                    if let Some(syms_part) = raw_imports.split('{').nth(1) {
                        let syms = syms_part.trim_end_matches('}');
                        for sym in syms.split(',') {
                            let sym_trim = sym.trim();
                            if sym_trim.is_empty() {
                                continue;
                            }
                            let (name, alias) = if sym_trim.contains(" as ") {
                                let parts: Vec<&str> = sym_trim.split(" as ").collect();
                                (parts[0].trim(), parts[1].trim())
                            } else {
                                (sym_trim, sym_trim)
                            };
                            let fullname = format!("{}.{}", mod_path, name);
                            data.imported_aliases
                                .insert(alias.to_string(), fullname.clone());
                            data.imports_list.push(ImportInfo {
                                line: LineNumber::new(idx),
                                module: ModuleName::new(fullname),
                                name: SymbolName::new(""),
                            });
                        }
                    }
                } else {
                    let alias = raw_imports;
                    data.imported_aliases
                        .insert(alias.to_string(), mod_path.clone());
                    data.imports_list.push(ImportInfo {
                        line: LineNumber::new(idx),
                        module: ModuleName::new(mod_path),
                        name: SymbolName::new(""),
                    });
                }
            } else if let Some(req_cap) = REQUIRE_REGEX.captures(stripped) {
                let alias = req_cap.get(1).unwrap().as_str().trim();
                let mod_path_raw = req_cap
                    .get(2)
                    .or_else(|| req_cap.get(3))
                    .unwrap()
                    .as_str()
                    .trim();
                let mod_path = mod_path_raw
                    .replace('/', ".")
                    .trim_start_matches('.')
                    .to_string();
                data.imported_aliases
                    .insert(alias.to_string(), mod_path.clone());
                data.imports_list.push(ImportInfo {
                    line: LineNumber::new(idx),
                    module: ModuleName::new(mod_path),
                    name: SymbolName::new(""),
                });
            }

            if let Some(fn_cap) = FN_REGEX_JS.captures(stripped) {
                let name = fn_cap.get(1).unwrap().as_str().to_string();
                data.defined.insert(name.clone());
                let col_pos = line.find(&name).unwrap_or(0) as i64;
                data.function_definitions.push(serde_json::json!({
                    "name": name,
                    "line": idx,
                    "column": col_pos,
                }));
            }

            if let Some(let_cap) = LET_REGEX_JS.captures(stripped) {
                let name = let_cap.get(1).unwrap().as_str().to_string();
                let col_pos = line.find(&name).unwrap_or(0) as i64;
                data.assignments.push(serde_json::json!({
                    "name": name,
                    "type": "Assign",
                    "line": idx,
                    "column": col_pos,
                }));
            }

            let cf_matches = CF_REGEX_JS.find_iter(stripped).count() as i64;
            data.control_flow_count += cf_matches;

            for cap in WORD_REGEX_JS.find_iter(stripped) {
                let word = cap.as_str();
                if !js_keywords.contains(word) && !word.chars().next().unwrap().is_numeric() {
                    data.used.insert(word.to_string());
                }
            }
        }

        Ok(data)
    }
}

impl ISourceParserPort for ASTJSParserAdapter {
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
            value: serde_json::json!(map),
            stdout: crate::taxonomy::StdOutput::new(""),
            stderr: crate::taxonomy::StdError::new(""),
            returncode: crate::taxonomy::ExitCode::new(0),
            metadata: crate::taxonomy::MetadataVO::new(HashMap::new()),
        })
    }

    fn get_class_attributes(&self, _path: &FilePath) -> ResponseData {
        ResponseData {
            value: serde_json::json!(HashMap::<String, serde_json::Value>::new()),
            stdout: crate::taxonomy::StdOutput::new(""),
            stderr: crate::taxonomy::StdError::new(""),
            returncode: crate::taxonomy::ExitCode::new(0),
            metadata: crate::taxonomy::MetadataVO::new(HashMap::new()),
        }
    }

    fn has_all_export(&self, path: &FilePath) -> SuccessStatus {
        let filename = path.value.replace('\\', "/");
        let is_barrel = filename.ends_with("/index.ts")
            || filename.ends_with("/index.js")
            || filename.ends_with("/index.tsx")
            || filename.ends_with("/index.jsx");
        if !is_barrel {
            return SuccessStatus {
                value: BooleanVO::new(false),
            };
        }
        if let Ok(data) = self.read_and_parse(path) {
            SuccessStatus {
                value: BooleanVO::new(!data.exported.is_empty()),
            }
        } else {
            SuccessStatus {
                value: BooleanVO::new(false),
            }
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

        let prim_regexes: Vec<(String, Regex)> = primitive_types
            .values
            .iter()
            .filter_map(|p| {
                let pattern = format!(r"\b{}\b", p.value);
                Regex::new(&pattern).ok().map(|re| (p.value.clone(), re))
            })
            .collect();

        for (idx, line) in content.lines().enumerate() {
            let stripped = line.trim();
            if stripped.starts_with("import ")
                || stripped.starts_with("//")
                || stripped.starts_with("/*")
            {
                continue;
            }

            let is_decl = stripped.contains("class ") || stripped.contains("constructor");
            if is_decl {
                for (prim, re) in &prim_regexes {
                    if let Some(m) = re.find(stripped) {
                        violations.push(PrimitiveViolation {
                            line: LineNumber::new((idx + 1) as i64),
                            column: ColumnNumber::new((m.start() + 1) as i64),
                            type_name: PrimitiveTypeName::new(prim.clone()),
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
            let mut found_use = false;

            if data.used.contains(&mod_name.value) || data.exported.contains(&mod_name.value) {
                found_use = true;
            } else {
                for (alias, full) in &data.imported_aliases {
                    if full == &mod_name.value
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
        if let Ok(data) = self.read_and_parse(path) {
            SuccessStatus {
                value: BooleanVO::new(data.exported.contains(&symbol.value)),
            }
        } else {
            SuccessStatus {
                value: BooleanVO::new(false),
            }
        }
    }

    fn get_class_methods(&self, path: &FilePath) -> MetadataVO {
        if let Ok(data) = self.read_and_parse(path) {
            let mut map = HashMap::new();
            map.insert(
                "methods".to_string(),
                serde_json::to_value(&data.class_methods).unwrap(),
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
                serde_json::to_value(&data.class_bases).unwrap(),
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

    fn is_barrel_file(&self, path: &FilePath) -> bool {
        let path_str = path.value.replace('\\', "/");
        path_str.ends_with("/index.ts")
            || path_str.ends_with("/index.js")
            || path_str.ends_with("/index.tsx")
            || path_str.ends_with("/index.jsx")
    }

    fn get_stem(&self, path: &FilePath) -> SymbolName {
        let basename = path
            .value
            .replace('\\', "/")
            .split('/')
            .last()
            .unwrap_or("")
            .to_string();
        let mut stem = basename.clone();
        for ext in &[".tsx", ".ts", ".jsx", ".js"] {
            if basename.ends_with(ext) {
                stem = basename[..basename.len() - ext.len()].to_string();
                break;
            }
        }
        SymbolName::new(stem)
    }

    fn is_entry_point(&self, path: &FilePath) -> bool {
        let basename = path
            .value
            .replace('\\', "/")
            .split('/')
            .last()
            .unwrap_or("")
            .to_string();
        [
            "index.ts",
            "index.js",
            "index.tsx",
            "index.jsx",
            "main.ts",
            "main.js",
        ]
        .contains(&basename.as_ref())
    }

    fn get_supported_extensions(&self) -> PatternList {
        PatternList::new(vec![
            ".js".to_string(),
            ".ts".to_string(),
            ".jsx".to_string(),
            ".tsx".to_string(),
        ])
    }
}

// ── Python Parser Adapter ────────────────────────────────────────────────────

pub struct ASTPythonParserAdapter;

impl ASTPythonParserAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl ISourceParserPort for ASTPythonParserAdapter {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
        Err(SourceParserError {
            path: path.clone(),
            message: ErrorMessage::new("Not yet implemented"),
            ..Default::default()
        })
    }

    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError> {
        Err(SourceParserError {
            path: path.clone(),
            message: ErrorMessage::new("Not yet implemented"),
            ..Default::default()
        })
    }

    fn get_class_attributes(&self, _path: &FilePath) -> ResponseData {
        ResponseData::new(serde_json::Value::Null)
    }

    fn has_all_export(&self, _path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(BooleanVO::new(false))
    }

    fn find_primitive_violations(
        &self,
        _path: &FilePath,
        _primitive_types: &PrimitiveTypeList,
    ) -> PrimitiveViolationList {
        PrimitiveViolationList::new()
    }

    fn find_unused_imports(&self, _path: &FilePath) -> ImportInfoList {
        ImportInfoList::new()
    }

    fn get_class_definitions(&self, _path: &FilePath) -> Result<MetadataVO, SourceParserError> {
        Ok(MetadataVO::new(std::collections::HashMap::new()))
    }

    fn get_function_definitions(&self, _path: &FilePath) -> MetadataVO {
        MetadataVO::new(std::collections::HashMap::new())
    }

    fn is_symbol_exported(&self, _path: &FilePath, _symbol: &SymbolName) -> SuccessStatus {
        SuccessStatus::new(BooleanVO::new(false))
    }

    fn get_class_methods(&self, _path: &FilePath) -> MetadataVO {
        MetadataVO::new(std::collections::HashMap::new())
    }

    fn get_class_bases_map(&self, _path: &FilePath) -> MetadataVO {
        MetadataVO::new(std::collections::HashMap::new())
    }

    fn get_assignment_targets(&self, _path: &FilePath) -> MetadataVO {
        MetadataVO::new(std::collections::HashMap::new())
    }

    fn get_control_flow_count(&self, _path: &FilePath) -> Count {
        Count::new(0)
    }

    fn is_barrel_file(&self, path: &FilePath) -> bool {
        path.value.ends_with("__init__.py")
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

    fn is_entry_point(&self, path: &FilePath) -> bool {
        path.value.ends_with("main.py") || path.value.ends_with("__main__.py")
    }

    fn get_supported_extensions(&self) -> PatternList {
        PatternList::new(vec![".py".to_string()])
    }
}

// ── Orchestrator ─────────────────────────────────────────────────────────────

pub struct SourceParserOrchestrator {
    python_parser: ASTPythonParserAdapter,
    rust_parser: ASTRustParserAdapter,
    js_parser: ASTJSParserAdapter,
}

impl SourceParserOrchestrator {
    pub fn new() -> Self {
        Self {
            python_parser: ASTPythonParserAdapter::new(),
            rust_parser: ASTRustParserAdapter::new(),
            js_parser: ASTJSParserAdapter::new(),
        }
    }

    fn select_parser(&self, path: &FilePath) -> &dyn ISourceParserPort {
        if path.value.ends_with(".rs") {
            return &self.rust_parser;
        }
        for ext in &[".ts", ".tsx", ".js", ".jsx"] {
            if path.value.ends_with(ext) {
                return &self.js_parser;
            }
        }
        &self.python_parser
    }
}

impl ISourceParserPort for SourceParserOrchestrator {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
        self.select_parser(path).extract_imports(path)
    }

    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError> {
        self.select_parser(path).get_raw_symbols(path)
    }

    fn get_class_attributes(&self, path: &FilePath) -> ResponseData {
        self.select_parser(path).get_class_attributes(path)
    }

    fn has_all_export(&self, path: &FilePath) -> SuccessStatus {
        self.select_parser(path).has_all_export(path)
    }

    fn find_primitive_violations(
        &self,
        path: &FilePath,
        primitive_types: &PrimitiveTypeList,
    ) -> PrimitiveViolationList {
        self.select_parser(path).find_primitive_violations(path, primitive_types)
    }

    fn find_unused_imports(&self, path: &FilePath) -> ImportInfoList {
        self.select_parser(path).find_unused_imports(path)
    }

    fn get_class_definitions(&self, path: &FilePath) -> Result<MetadataVO, SourceParserError> {
        self.select_parser(path).get_class_definitions(path)
    }

    fn get_function_definitions(&self, path: &FilePath) -> MetadataVO {
        self.select_parser(path).get_function_definitions(path)
    }

    fn is_symbol_exported(&self, path: &FilePath, symbol: &SymbolName) -> SuccessStatus {
        self.select_parser(path).is_symbol_exported(path, symbol)
    }

    fn get_class_methods(&self, path: &FilePath) -> MetadataVO {
        self.select_parser(path).get_class_methods(path)
    }

    fn get_class_bases_map(&self, path: &FilePath) -> MetadataVO {
        self.select_parser(path).get_class_bases_map(path)
    }

    fn get_assignment_targets(&self, path: &FilePath) -> MetadataVO {
        self.select_parser(path).get_assignment_targets(path)
    }

    fn get_control_flow_count(&self, path: &FilePath) -> Count {
        self.select_parser(path).get_control_flow_count(path)
    }

    fn is_barrel_file(&self, path: &FilePath) -> bool {
        self.select_parser(path).is_barrel_file(path)
    }

    fn get_stem(&self, path: &FilePath) -> SymbolName {
        self.select_parser(path).get_stem(path)
    }

    fn is_entry_point(&self, path: &FilePath) -> bool {
        self.select_parser(path).is_entry_point(path)
    }

    fn get_supported_extensions(&self) -> PatternList {
        PatternList::new(vec![
            ".py".to_string(),
            ".rs".to_string(),
            ".ts".to_string(),
            ".tsx".to_string(),
            ".js".to_string(),
            ".jsx".to_string(),
        ])
    }
}
