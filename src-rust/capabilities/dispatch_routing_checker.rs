// dispatch_routing_checker — Static analysis for MCP/server dispatch routing.
// 1:1 Rust implementation matching capabilities/dispatch_routing_checker.py
//
// Detects:
//   AES030 — Capability method referenced in COMMAND_CATALOG doesn't exist on the class
//   AES031 — Orchestrator routes ALL actions to a single capability when other options exist
//   AES032 — Capability method called without required request VO parameter

use regex::Regex;
use once_cell::sync::Lazy;

use crate::taxonomy::{
    AdapterName, CapabilityReference, CapabilityReferenceList, CapabilityRoutingContext,
    ClassDefinitionMap, ClassFileMap, ClassUsageItem, ClassUsageItemList,
    ClassUsageMap, ErrorCode, FilePath, LineNumber,
    LintMessage, LintResult, Severity,
};
use crate::contract::arch_rule_protocol::IAnalyzer;
use crate::contract::dispatch_routing_protocol::IDispatchRoutingParserProtocol;
use super::dispatch_parser_types::MethodArgsVO;
use super::dispatch_routing_parser::DispatchRoutingParser;

static CAPABILITY_REF_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"["']capability["']\s*:\s*["']([A-Za-z_][\w]*)\.([A-Za-z_][\w]*)["']"#).unwrap()
});

pub struct DispatchRoutingChecker {
    pub parser: Box<dyn IDispatchRoutingParserProtocol>,
}

impl DispatchRoutingChecker {
    pub fn new() -> Self {
        Self {
            parser: Box::new(DispatchRoutingParser::new()),
        }
    }

    pub fn with_parser(parser: Box<dyn IDispatchRoutingParserProtocol>) -> Self {
        Self { parser }
    }

    pub async fn check_capability_routing(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &crate::taxonomy::FilePathList,
        root_dir: &FilePath,
        results: &mut crate::taxonomy::LintResultList,
    ) {
        let context = self._check_capability_by_layer(analyzer, files);

        self._verify_capability_references(&context.references, &context.definitions, results);

        self._check_single_capability_bottleneck(&context.references, &context.definitions, results, root_dir);

        let cap_files: Vec<FilePath> = context.references.references.iter()
            .map(|r| r.file.clone())
            .collect();
        if !cap_files.is_empty() {
            self._check_missing_vo_construction(analyzer, &cap_files, results);
        }
    }

    fn _check_capability_by_layer(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &crate::taxonomy::FilePathList,
    ) -> CapabilityRoutingContext {
        let mut capability_refs = CapabilityReferenceList { references: vec![] };
        let mut class_defs = ClassDefinitionMap {
            definitions: std::collections::HashMap::new(),
        };
        let mut class_files = ClassFileMap {
            mapping: std::collections::HashMap::new(),
        };

        for f in &files.values {
            let path = f.to_string();
            if !path.ends_with(".py") {
                continue;
            }

            let text = match self._read_file_content(analyzer, f) {
                Some(t) => t,
                None => continue,
            };

            let stripped_text = self.parser.strip_docstrings(&text);

            self._collect_capability_refs(&stripped_text, f, &mut capability_refs);

            let class_info = self.parser.extract_class_methods(&stripped_text);
            for (cls_name, methods_vo) in &class_info.definitions {
                if !class_defs.definitions.contains_key(cls_name) {
                    class_defs.definitions.insert(cls_name.clone(), methods_vo.clone());
                    class_files.mapping.insert(cls_name.clone(), f.clone());
                }
            }
        }

        CapabilityRoutingContext {
            references: capability_refs,
            definitions: class_defs,
            files: class_files,
        }
    }

    fn _read_file_content(&self, analyzer: &dyn IAnalyzer, file_path: &FilePath) -> Option<String> {
        std::fs::read_to_string(file_path.to_string().as_str()).ok()
    }

    fn _collect_capability_refs(
        &self,
        text: &str,
        file_path: &FilePath,
        refs: &mut CapabilityReferenceList,
    ) {
        for mat in CAPABILITY_REF_PATTERN.find_iter(text) {
            let class_name = mat.get(1).unwrap().as_str().to_string();
            let method_name = mat.get(2).unwrap().as_str().to_string();
            let line_no = text[..mat.start()].chars().filter(|&c| c == '\n').count() as i64 + 1;
            refs.references.push(CapabilityReference {
                file: file_path.clone(),
                line: LineNumber::new(line_no),
                class_name,
                method_name,
            });
        }
    }

    fn _verify_capability_references(
        &self,
        capability_refs: &CapabilityReferenceList,
        class_defs: &ClassDefinitionMap,
        results: &mut crate::taxonomy::LintResultList,
    ) {
        for ref_ in &capability_refs.references {
            if let Some(class_methods) = class_defs.definitions.get(&ref_.class_name) {
                if !class_methods.methods.contains(&ref_.method_name) {
                    let found_methods = if class_methods.methods.is_empty() {
                        "(none)".to_string()
                    } else {
                        class_methods.methods.join(", ")
                    };
                    self._report(
                        results,
                        &ref_.file,
                        &ref_.line,
                        "AES030",
                        &format!(
                            "Method '{}' not found on class '{}'. Defined methods: {}. Check for naming mismatch between catalog and capability.",
                            ref_.method_name, ref_.class_name, found_methods
                        ),
                    );
                }
            } else {
                self._report(
                    results,
                    &ref_.file,
                    &ref_.line,
                    "AES030",
                    &format!(
                        "Capability class '{}' not found in any scanned file. Referenced from COMMAND_CATALOG but no class definition exists.",
                        ref_.class_name
                    ),
                );
            }
        }
    }

    fn _check_single_capability_bottleneck(
        &self,
        capability_refs: &CapabilityReferenceList,
        class_defs: &ClassDefinitionMap,
        results: &mut crate::taxonomy::LintResultList,
        _root_dir: &FilePath,
    ) {
        if capability_refs.references.is_empty() {
            return;
        }

        let class_usage = self._group_capabilities_by_class(capability_refs);

        if class_usage.usage.len() == 1 {
            let (single_class, usage_list) = class_usage.usage.iter().next().unwrap();
            if !usage_list.items.is_empty() {
                let other_classes: Vec<String> = class_defs.definitions.keys()
                    .filter(|c| c != single_class)
                    .cloned()
                    .collect();
                if !other_classes.is_empty() && usage_list.items.len() >= 3 {
                    self._report_class_bottleneck(results, single_class, usage_list, &other_classes);
                }
            }
        }
    }

    fn _group_capabilities_by_class(
        &self,
        capability_refs: &CapabilityReferenceList,
    ) -> ClassUsageMap {
        let mut class_usage = ClassUsageMap {
            usage: std::collections::HashMap::new(),
        };
        for ref_ in &capability_refs.references {
            class_usage.usage.entry(ref_.class_name.clone())
                .or_insert_with(|| ClassUsageItemList { items: vec![] })
                .items.push(ClassUsageItem {
                    file: ref_.file.clone(),
                    line: ref_.line.clone(),
                    method: ref_.method_name.clone(),
                });
        }
        class_usage
    }

    fn _report_class_bottleneck(
        &self,
        results: &mut crate::taxonomy::LintResultList,
        class_name: &str,
        refs: &ClassUsageItemList,
        other_classes: &[String],
    ) {
        let other_names: Vec<String> = other_classes.iter().take(5).cloned().collect();
        let other_str = other_names.join(", ");
        for item in &refs.items {
            self._report(
                results,
                &item.file,
                &item.line,
                "AES031",
                &format!(
                    "Action '{}' routes to '{}' but {} other capability classes exist ({}). Actions should be distributed to the correct capability.",
                    item.method, class_name, other_classes.len(), other_str
                ),
            );
        }
    }

    fn _check_missing_vo_construction(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &[FilePath],
        results: &mut crate::taxonomy::LintResultList,
    ) {
        for f in files {
            self._check_file_vo_construction(analyzer, f, results);
        }
    }

    fn _check_file_vo_construction(
        &self,
        _analyzer: &dyn IAnalyzer,
        file_path: &FilePath,
        results: &mut crate::taxonomy::LintResultList,
    ) {
        let path = file_path.to_string();
        if !path.ends_with(".py") {
            return;
        }

        let content = match std::fs::read_to_string(path.as_str()) {
            Ok(c) => c,
            Err(_) => return,
        };

        static CALL_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?:await\s+)?self\.\w+\.(\w+)\s*\(").unwrap()
        });

        for mat in CALL_PATTERN.find_iter(&content) {
            let method_name = mat.get(1).unwrap().as_str();
            let paren_start = mat.end() - 1;
            let args_vo = self._extract_args(&content, paren_start);
            if let Some(ref args_val) = args_vo.value {
                let args_text = args_val.trim();
                if args_text.is_empty() {
                    let line_no = content[..paren_start].chars().filter(|&c| c == '\n').count() as i64 + 1;
                    self._report(
                        results,
                        file_path,
                        &LineNumber::new(line_no),
                        "AES032",
                        &format!(
                            "Capability call 'self.some_executor.{}()' missing required request/data VO parameter. Capability methods expect a typed Value Object argument.",
                            method_name
                        ),
                    );
                }
            }
        }
    }

    fn _extract_args(&self, content: &str, open_paren: usize) -> MethodArgsVO {
        if open_paren >= content.len() || content.as_bytes()[open_paren] != b'(' {
            return MethodArgsVO { value: None };
        }
        let mut depth = 1i32;
        let mut i = open_paren + 1;
        let bytes = content.as_bytes();
        while i < content.len() && depth > 0 {
            match bytes[i] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                _ => {}
            }
            i += 1;
        }
        if depth == 0 {
            MethodArgsVO { value: Some(content[open_paren + 1..i - 1].to_string()) }
        } else {
            MethodArgsVO { value: None }
        }
    }

    fn _report(
        &self,
        results: &mut crate::taxonomy::LintResultList,
        file: &FilePath,
        line: &LineNumber,
        code: &str,
        message: &str,
    ) {
        results.push(LintResult {
            file: file.clone(),
            line: line.clone(),
            column: ColumnNumber::new(1),
            code: ErrorCode::new(code),
            message: LintMessage::new(message),
            severity: Severity::MEDIUM,
            source: AdapterName::new("dispatch_routing").unwrap_or_default(),
            enclosing_scope: crate::taxonomy::ScopeRef::new(""),
            related_locations: crate::taxonomy::LocationList::new(),
        });
    }
}
