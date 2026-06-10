// PURPOSE: RustScanner — ISourceParserPort for Rust use/mod import extraction
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::LazyLock;

use crate::code_analysis::taxonomy_import_source_vo::ImportInfo;
use crate::code_analysis::taxonomy_import_source_vo::ImportInfoList;
use crate::code_analysis::taxonomy_import_source_vo::PrimitiveViolation;
use crate::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList;
use crate::shared_common::taxonomy_name_vo::SymbolName;
use crate::shared_common::taxonomy_naming_list_vo::PrimitiveTypeList;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::shared_common::taxonomy_common_error::Cause;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_common_vo::PatternList;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_suggestion_vo::MetadataVO;
use crate::source_parsing::contract_parser_port::ISourceParserPort;
use crate::source_parsing::taxonomy_parser_error::SourceParserError;
use crate::source_parsing::taxonomy_path_vo::FilePath;

static USE_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^(?:pub\s+)?use\s+([^;]+);").ok());
static STRUCT_REGEX: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(r"^(?:pub\s+)?(?:pub\s*\([^)]*\)\s+)?(?:struct|enum|trait)\s+([a-zA-Z0-9_]+)").ok()
});
static FN_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^(?:pub\s+)?(?:async\s+)?fn\s+([a-zA-Z0-9_]+)").ok());
static IMPL_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^impl\s+(?:([a-zA-Z0-9_:]+)\s+for\s+)?([a-zA-Z0-9_]+)").ok());
static CF_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"\b(if|for|while|match|loop)\b").ok());
static LET_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^let\s+(?:mut\s+)?([a-zA-Z0-9_]+)").ok());
static WORD_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").ok());

static PUB_STRUCT_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"\b(struct|enum|trait|fn|const)\s+([a-zA-Z0-9_]+)").ok());
static PUB_MOD_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"\bmod\s+([a-zA-Z0-9_]+)").ok());
static PUB_USE_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"\buse\s+(?:.*::)?([a-zA-Z0-9_]+)\s*(?:::\{|;|$)").ok());
static PUB_USE_GROUP: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"\buse\s+.*::\{([^}]+)\}").ok());
static TYPE_DECL_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"\b(struct|enum|trait|fn|impl|pub)\b").ok());

pub struct ASTRustParserAdapter {}

impl Default for ASTRustParserAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ASTRustParserAdapter {
    pub fn new() -> Self {
        Self {}
    }

    fn read_and_parse(&self, path: &FilePath) -> Result<ParsedData, SourceParserError> {
        let content = fs::read_to_string(&path.value).map_err(|e| SourceParserError {
            path: path.clone(),
            message: ErrorMessage::new(format!("Failed to read file: {}", e)),
            error_code: ErrorCode::raw("FILE_READ_ERROR"),
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
            if stripped.is_empty()
                || stripped.starts_with("//")
                || stripped.starts_with("/*")
                || stripped.starts_with("*")
            {
                continue;
            }

            let open_braces = stripped.matches('{').count() as i32;
            let close_braces = stripped.matches('}').count() as i32;

            // Check for impl block
            if let Some(impl_cap) = IMPL_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                let trait_name = impl_cap.get(1).map(|m| m.as_str());
                let struct_name = impl_cap
                    .get(2)
                    .map(|m| m.as_str())
                    .unwrap_or("")
                    .to_string();

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

            // 1. Imports — handle single-line and multi-line use statements
            if stripped.starts_with("use ") || stripped.starts_with("pub use ") {
                let mut full_use = stripped.to_string();
                // Multi-line: keep reading subsequent lines until `;` is found
                if full_use.contains('{') && !full_use.contains(';') {
                    let mut j = idx_zero + 1;
                    while j < lines.len() {
                        let cont = lines[j].trim();
                        full_use.push(' ');
                        full_use.push_str(cont);
                        if cont.contains(';') {
                            break;
                        }
                        j += 1;
                    }
                }
                if let Some(use_cap) = USE_REGEX.as_ref().and_then(|r| r.captures(&full_use)) {
                    let raw_path = use_cap.get(1).map(|m| m.as_str()).unwrap_or("").trim();
                    let expanded = if raw_path.contains("::{") {
                        // Split on ::{ to get prefix group and braced sub-items
                        if let Some(brace_pos) = raw_path.find("::{") {
                            let mut prefix = &raw_path[..brace_pos];
                            // Strip crate::/self::/super:: from prefix only
                            for p in &["crate::", "self::", "super::"] {
                                if prefix.starts_with(p) {
                                    prefix = &prefix[p.len()..];
                                    break;
                                }
                            }
                            let rest = &raw_path[brace_pos + 3..];
                            let inner = rest.trim_start_matches('{').trim_end_matches('}').trim();
                            inner
                                .split(',')
                                .map(|s| format!("{}::{}", prefix, s.trim()))
                                .filter(|p| !p.is_empty())
                                .collect::<Vec<String>>()
                        } else {
                            vec![raw_path.to_string()]
                        }
                    } else {
                        let mut clean_path = raw_path;
                        for prefix in &["crate::", "self::", "super::"] {
                            if clean_path.starts_with(prefix) {
                                clean_path = &clean_path[prefix.len()..];
                                break;
                            }
                        }
                        vec![clean_path.to_string()]
                    };

                    for item in &expanded {
                        let dotted = item.replace("::", ".");
                        let alias = dotted.split('.').next_back().unwrap_or(&dotted).to_string();
                        if alias != "*" {
                            imported_aliases.insert(alias, dotted.clone());
                            imports_list.push(ImportInfo {
                                line: LineNumber::new(idx),
                                module: dotted,
                                name: None,
                            });
                        }
                    }
                }
            }

            // 2. Struct, Enum, Trait Definitions
            if let Some(struct_cap) = STRUCT_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                let name = struct_cap
                    .get(1)
                    .map(|m| m.as_str())
                    .unwrap_or("")
                    .to_string();
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

            // 3. Functions / Methods
            if let Some(fn_cap) = FN_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                let name = fn_cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                defined.insert(name.clone());

                if let Some(ref cimpl) = current_impl {
                    class_methods.entry(cimpl.clone()).or_default().push(name);
                } else {
                    let col_pos = line.find(&name).unwrap_or(0) as i64;
                    func_defs.push(serde_json::json!({
                        "name": name,
                        "line": idx,
                        "column": col_pos
                    }));
                }
            }

            // 4. Assignments
            if stripped.starts_with("let ") {
                if let Some(let_cap) = LET_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                    let name = let_cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                    let col_pos = line.find(&name).unwrap_or(0) as i64;
                    assignments.push(serde_json::json!({
                        "name": name,
                        "type": "Assign",
                        "line": idx,
                        "column": col_pos
                    }));
                }
            }

            // 5. Control Flow
            control_flow_count += CF_REGEX
                .as_ref()
                .map_or(0, |r| r.find_iter(stripped).count())
                as i64;

            // 6. Used symbols
            if let Some(word_re) = WORD_REGEX.as_ref() {
                for word_match in word_re.find_iter(stripped) {
                    used.insert(word_match.as_str().to_string());
                }
            }

            // 7. Exported symbols (pub items)
            if stripped.starts_with("pub ") {
                if let Some(cap) = PUB_STRUCT_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                    exported.insert(cap.get(2).map(|m| m.as_str()).unwrap_or("").to_string());
                }
                if let Some(cap) = PUB_MOD_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                    exported.insert(cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string());
                }
                if let Some(cap) = PUB_USE_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                    exported.insert(cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string());
                }
                if let Some(cap) = PUB_USE_GROUP.as_ref().and_then(|r| r.captures(stripped)) {
                    for name in cap.get(1).map(|m| m.as_str()).unwrap_or("").split(',') {
                        let clean = name.trim();
                        if !clean.is_empty() {
                            exported.insert(clean.to_string());
                        }
                    }
                }
            }
        }

        // Post-processing class definitions to add bases and resolved_bases
        for cdef in &mut class_defs {
            if let Some(obj) = cdef.as_object_mut() {
                if let Some(cname_val) = obj.get("name").and_then(|v| v.as_str()) {
                    let cname = cname_val.to_string();
                    if let Some(bases) = class_bases.get(&cname) {
                        obj.insert("bases".to_string(), serde_json::json!(bases));
                        let resolved: Vec<String> = bases
                            .iter()
                            .map(|b| {
                                imported_aliases
                                    .get(b)
                                    .cloned()
                                    .unwrap_or_else(|| b.clone())
                            })
                            .collect();
                        obj.insert("resolved_bases".to_string(), serde_json::json!(resolved));
                    }
                }
            }
        }

        Ok(ParsedData {
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

struct ParsedData {
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

impl ISourceParserPort for ASTRustParserAdapter {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
        let data = self.read_and_parse(path)?;
        Ok(ImportInfoList {
            values: data.imports_list,
        })
    }

    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError> {
        let data = self.read_and_parse(path)?;
        let mut map = HashMap::new();
        map.insert("defined".to_string(), serde_json::json!(data.defined));
        map.insert("used".to_string(), serde_json::json!(data.used));
        map.insert("exported".to_string(), serde_json::json!(data.exported));
        map.insert("aliases".to_string(), serde_json::json!(data.aliases));
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
            let mut in_struct = false;
            let mut struct_name = String::new();
            for line in &lines {
                let stripped = line.trim();
                if let Some(cap) = STRUCT_REGEX.as_ref().and_then(|r| r.captures(stripped)) {
                    struct_name = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                    in_struct = true;
                    continue;
                }
                if in_struct {
                    if stripped.starts_with('}')
                        || stripped.starts_with("//")
                        || stripped.is_empty()
                    {
                        if stripped.starts_with('}') {
                            in_struct = false;
                        }
                        continue;
                    }
                    if stripped.contains(':')
                        && !stripped.starts_with("fn ")
                        && !stripped.starts_with("impl ")
                    {
                        let field_name = stripped
                            .split(':')
                            .next()
                            .unwrap_or("")
                            .trim()
                            .trim_start_matches("pub ")
                            .to_string();
                        if !field_name.is_empty() && !field_name.contains(' ') {
                            attrs
                                .entry(struct_name.clone())
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
            returncode: 0i64,
            metadata: HashMap::new(),
        }
    }

    fn has_all_export(&self, path: &FilePath) -> SuccessStatus {
        if !self.is_barrel_file(path).value() {
            return SuccessStatus { value: false };
        }
        if let Ok(data) = self.read_and_parse(path) {
            SuccessStatus {
                value: !data.exported.is_empty(),
            }
        } else {
            SuccessStatus { value: false }
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
            if stripped.starts_with("use ")
                || stripped.starts_with("//")
                || stripped.starts_with("/*")
            {
                continue;
            }

            if TYPE_DECL_REGEX
                .as_ref()
                .is_some_and(|r| r.is_match(stripped))
            {
                for prim in &prim_keywords {
                    let Ok(prim_regex) = Regex::new(&format!(r"\b{}\b", prim)) else {
                        continue;
                    };
                    if let Some(m) = prim_regex.find(stripped) {
                        let col = (line.find(m.as_str()).unwrap_or(0) + 1) as i64;
                        violations.push(PrimitiveViolation {
                            line: LineNumber::new(idx),
                            column: ColumnNumber::new(col),
                            type_name: prim.clone(),
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
            let mod_name = imp.module.clone();
            let mut is_used = used_set.contains(&mod_name) || exported_set.contains(&mod_name);

            if !is_used {
                for (alias, fullname) in &data.aliases {
                    if fullname == &mod_name
                        && (used_set.contains(alias) || exported_set.contains(alias))
                    {
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
        map.insert(
            "classes".to_string(),
            serde_json::json!(data.class_definitions),
        );
        Ok(MetadataVO { value: map })
    }

    fn get_function_definitions(&self, path: &FilePath) -> MetadataVO {
        let mut map = HashMap::new();
        if let Ok(data) = self.read_and_parse(path) {
            map.insert(
                "functions".to_string(),
                serde_json::json!(data.function_definitions),
            );
        } else {
            map.insert(
                "functions".to_string(),
                serde_json::json!(Vec::<serde_json::Value>::new()),
            );
        }
        MetadataVO { value: map }
    }

    fn is_symbol_exported(&self, path: &FilePath, symbol: &SymbolName) -> SuccessStatus {
        if let Ok(data) = self.read_and_parse(path) {
            SuccessStatus {
                value: data.exported.contains(&symbol.value),
            }
        } else {
            SuccessStatus { value: false }
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
            map.insert(
                "assignments".to_string(),
                serde_json::json!(data.assignments),
            );
        } else {
            map.insert(
                "assignments".to_string(),
                serde_json::json!(Vec::<serde_json::Value>::new()),
            );
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

    fn is_barrel_file(&self, path: &FilePath) -> BooleanVO {
        let path_str = path.value.replace('\\', "/");
        BooleanVO::new(path_str.ends_with("/mod.rs") || path_str.ends_with("/lib.rs"))
    }

    fn get_stem(&self, path: &FilePath) -> SymbolName {
        let path_str = path.value.replace('\\', "/");
        let basename = path_str.split('/').next_back().unwrap_or(&path.value);
        SymbolName::new(basename.replace(".rs", ""))
    }

    fn is_entry_point(&self, path: &FilePath) -> BooleanVO {
        let path_str = path.value.replace('\\', "/");
        let basename = path_str.split('/').next_back().unwrap_or(&path.value);
        BooleanVO::new(basename == "main.rs" || basename == "lib.rs" || basename == "mod.rs")
    }

    fn get_supported_extensions(&self) -> PatternList {
        PatternList::new(".rs")
    }
}
