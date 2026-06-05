use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::LazyLock;

use crate::contract::source_parser_port::ISourceParserPort;
use crate::taxonomy::{
    BooleanVO, Cause, ColumnNumber, Count, ErrorCode, ErrorMessage, FilePath, ImportInfo,
    ImportInfoList, LineNumber, MetadataVO, ModuleName, PatternList, PrimitiveTypeList,
    PrimitiveTypeName, PrimitiveViolation, PrimitiveViolationList, ResponseData, SourceParserError,
    SuccessStatus, SymbolName,
};

static IMPORT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^import\s+(.+?)\s+from\s+'([^']+)'").unwrap());
static IMPORT_DOUBLE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^import\s+(.+?)\s+from\s+"([^"]+)""#).unwrap());
static REQUIRE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^(?:const|let|var)\s+(\w+)\s*=\s*require\((?:'([^']+)'|"([^"]+)")\)"#).unwrap()
});
static CLASS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^class\s+(\w+)(?:\s+extends\s+(\w+))?").unwrap());
static FN_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:async\s+)?function\s+(\w+)").unwrap());
static CF_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(if|for|while|switch|catch)\b").unwrap());
static LET_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:const|let|var)\s+(\w+)\s*=").unwrap());
static METHOD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:async\s+)?(\w+)\s*\([^)]*\)\s*\{").unwrap());
static WORD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap());

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
}

pub struct ASTJSParserAdapter;

impl ASTJSParserAdapter {
    pub fn new() -> Self {
        Self
    }

    fn read_and_parse(&self, path: &FilePath) -> Result<ParsedData, SourceParserError> {
        let content = fs::read_to_string(&path.value).map_err(|e| SourceParserError {
            path: path.clone(),
            message: ErrorMessage::new(format!("Failed to read file: {}", e)),
            error_code: ErrorCode::new("FILE_READ_ERROR"),
            cause: Cause::new(e.to_string()),
        })?;

        let mut data = ParsedData::default();
        let lines: Vec<&str> = content.lines().collect();

        let mut in_comment = false;
        let mut current_class: Option<String> = None;
        let mut brace_count = 0;

        let js_keywords: HashSet<&str> = [
            "break",
            "case",
            "catch",
            "class",
            "const",
            "continue",
            "debugger",
            "default",
            "delete",
            "do",
            "else",
            "export",
            "extends",
            "finally",
            "for",
            "function",
            "if",
            "import",
            "in",
            "instanceof",
            "new",
            "return",
            "super",
            "switch",
            "this",
            "throw",
            "try",
            "typeof",
            "var",
            "void",
            "while",
            "with",
            "yield",
            "let",
            "static",
            "async",
            "await",
            "from",
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
                let name = class_cap.get(1).unwrap().as_ref().to_string();
                data.defined.insert(name.clone());

                let base = class_cap.get(2).map(|m| m.as_ref().to_string());
                let resolved_base = base
                    .as_ref()
                    .map(|b| data.imported_aliases.get(b).cloned().unwrap_or(b.clone()));

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
                        let mname = m_cap.get(1).unwrap().as_ref().to_string();
                        if !["if", "for", "while", "switch"].contains(&mname.as_ref()) {
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
                raw_imports = imp_cap.get(1).unwrap().as_ref().trim();
                module_path = imp_cap.get(2).unwrap().as_ref().trim();
                import_found = true;
            } else if let Some(imp_cap) = IMPORT_DOUBLE_REGEX.captures(stripped) {
                raw_imports = imp_cap.get(1).unwrap().as_ref().trim();
                module_path = imp_cap.get(2).unwrap().as_ref().trim();
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
                let alias = req_cap.get(1).unwrap().as_ref().trim();
                let mod_path_raw = req_cap
                    .get(2)
                    .or_else(|| req_cap.get(3))
                    .unwrap()
                    .as_ref()
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

            if let Some(fn_cap) = FN_REGEX.captures(stripped) {
                let name = fn_cap.get(1).unwrap().as_ref().to_string();
                data.defined.insert(name.clone());
                let col_pos = line.find(&name).unwrap_or(0) as i64;
                data.function_definitions.push(serde_json::json!({
                    "name": name,
                    "line": idx,
                    "column": col_pos,
                }));
            }

            if let Some(let_cap) = LET_REGEX.captures(stripped) {
                let name = let_cap.get(1).unwrap().as_ref().to_string();
                let col_pos = line.find(&name).unwrap_or(0) as i64;
                data.assignments.push(serde_json::json!({
                    "name": name,
                    "type": "Assign",
                    "line": idx,
                    "column": col_pos,
                }));
            }

            let cf_matches = CF_REGEX.find_iter(stripped).count() as i64;
            data.control_flow_count += cf_matches;

            for cap in WORD_REGEX.find_iter(stripped) {
                let word = cap.as_ref();
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

            if data.used.contains(&mod_name) || data.exported.contains(&mod_name) {
                found_use = true;
            } else {
                for (alias, full) in &data.imported_aliases {
                    if full == &mod_name
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
