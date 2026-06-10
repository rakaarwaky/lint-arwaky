// PURPOSE: Processor: Routing processing logic

//
// Detects:
//   AES0303 — Capability method referenced in COMMAND_CATALOG doesn't exist on the class
//   AES0303 — Orchestrator routes ALL actions to a single capability when other options exist
//   AES0303 — Capability method called without required request VO parameter

use once_cell::sync::Lazy;
use regex::Regex;

use crate::cli_commands::contract_routing_protocol::{
    IDispatchRoutingParserProtocol, IDispatchRoutingProtocol,
};
use crate::di_containers::taxonomy_routing_vo::CapabilityReference;
use crate::di_containers::taxonomy_routing_vo::CapabilityReferenceList;
use crate::di_containers::taxonomy_routing_vo::CapabilityRoutingContext;
use crate::di_containers::taxonomy_routing_vo::ClassDefinitionMap;
use crate::di_containers::taxonomy_routing_vo::ClassFileMap;
use crate::di_containers::taxonomy_routing_vo::ClassNameVO;
use crate::di_containers::taxonomy_routing_vo::ClassUsageItem;
use crate::di_containers::taxonomy_routing_vo::ClassUsageItemList;
use crate::di_containers::taxonomy_routing_vo::ClassUsageMap;
use crate::layer_rules::contract_rule_protocol::IAnalyzer;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_source_vo::ContentString;
use crate::shared_common::taxonomy_suggestion_vo::DescriptionVO;
use crate::shared_common::taxonomy_violation_rs_constant::{
    AES0303_CAPABILITY_ROUTING, AES0303_MISSING_VO, AES0303_SINGLE_BOTTLENECK,
};
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

/// Extracted method args VO (inlined from deleted dispatch_parser_types module).
pub struct MethodArgsVO {
    pub value: Option<String>,
}

/// Default parser implementation (inlined from deleted dispatch_routing_parser module).
pub struct DispatchRoutingParser {}

impl Default for DispatchRoutingParser {
    fn default() -> Self {
        Self::new()
    }
}

impl DispatchRoutingParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl IDispatchRoutingParserProtocol for DispatchRoutingParser {
    fn strip_docstrings(&self, text: &ContentString) -> ContentString {
        let re = match regex::Regex::new(r#""""[\s\S]*?""""|'''[\s\S]*?'''"|#[^\n]*"#) {
            Ok(r) => r,
            Err(_) => return ContentString::new(text.value.clone()),
        };
        ContentString::new(re.replace_all(&text.value, "").to_string())
    }

    fn extract_class_methods(
        &self,
        text: &ContentString,
    ) -> crate::di_containers::taxonomy_routing_vo::ClassDefinitionMap {
        use std::collections::HashMap;
        let mut defs: HashMap<
            ClassNameVO,
            crate::di_containers::taxonomy_routing_vo::ClassMethodsVO,
        > = HashMap::new();
        let class_re = match regex::Regex::new(r"class\s+(\w+)") {
            Ok(r) => r,
            Err(_) => {
                return crate::di_containers::taxonomy_routing_vo::ClassDefinitionMap {
                    definitions: HashMap::new(),
                }
            }
        };
        let method_re = match regex::Regex::new(r"def\s+(\w+)\s*\(") {
            Ok(r) => r,
            Err(_) => {
                return crate::di_containers::taxonomy_routing_vo::ClassDefinitionMap {
                    definitions: HashMap::new(),
                }
            }
        };
        let mut current_class: Option<ClassNameVO> = None;
        for line in text.value.lines() {
            if let Some(c) = class_re.captures(line) {
                let cls_name = ClassNameVO::new(c[1].to_string());
                current_class = Some(cls_name.clone());
                defs.entry(cls_name).or_insert_with(|| {
                    crate::di_containers::taxonomy_routing_vo::ClassMethodsVO { methods: vec![] }
                });
            } else if let Some(m) = method_re.captures(line) {
                if let Some(ref cls) = current_class {
                    if let Some(entry) = defs.get_mut(cls) {
                        entry.methods.push(DescriptionVO::new(m[1].to_string()));
                    }
                }
            }
        }
        crate::di_containers::taxonomy_routing_vo::ClassDefinitionMap { definitions: defs }
    }
}

static CAPABILITY_REF_PATTERN: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(r#"["']capability["']\s*:\s*["']([A-Za-z_][\w]*)\.([A-Za-z_][\w]*)["']"#).ok()
});

pub struct DispatchRoutingChecker {
    pub parser: Box<dyn IDispatchRoutingParserProtocol>,
}

impl Default for DispatchRoutingChecker {
    fn default() -> Self {
        Self::new()
    }
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
        files: &crate::source_parsing::taxonomy_paths_vo::FilePathList,
        root_dir: &FilePath,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        let context = self._check_capability_by_layer(analyzer, files);

        self._verify_capability_references(&context.references, &context.definitions, results);

        self._check_single_capability_bottleneck(
            &context.references,
            &context.definitions,
            results,
            root_dir,
        );

        let cap_files: Vec<FilePath> = context
            .references
            .references
            .iter()
            .map(|r| r.file.clone())
            .collect();
        if !cap_files.is_empty() {
            self._check_missing_vo_construction(analyzer, &cap_files, results);
        }
    }

    fn _check_capability_by_layer(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &crate::source_parsing::taxonomy_paths_vo::FilePathList,
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

            let text_cs = ContentString::new(text);
            let stripped_text = self.parser.strip_docstrings(&text_cs);

            self._collect_capability_refs(&stripped_text.value, f, &mut capability_refs);

            let class_info = self.parser.extract_class_methods(&stripped_text);
            for (cls_name, methods_vo) in &class_info.definitions {
                if !class_defs.definitions.contains_key(cls_name) {
                    class_defs
                        .definitions
                        .insert(cls_name.clone(), methods_vo.clone());
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

    fn _read_file_content(
        &self,
        _analyzer: &dyn IAnalyzer,
        file_path: &FilePath,
    ) -> Option<String> {
        std::fs::read_to_string(file_path.to_string().as_str()).ok()
    }

    fn _collect_capability_refs(
        &self,
        text: &str,
        file_path: &FilePath,
        refs: &mut CapabilityReferenceList,
    ) {
        let pattern = match &*CAPABILITY_REF_PATTERN {
            Some(r) => r,
            None => return,
        };
        for caps in pattern.captures_iter(text) {
            let class_name =
                ClassNameVO::new(caps.get(1).map(|m| m.as_str()).unwrap_or("").to_string());
            let method_name =
                DescriptionVO::new(caps.get(2).map(|m| m.as_str()).unwrap_or("").to_string());
            let line_no = text[..caps.get(0).map(|m| m.start()).unwrap_or(0)]
                .chars()
                .filter(|&c| c == '\n')
                .count() as i64
                + 1;
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
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        for ref_ in &capability_refs.references {
            if let Some(class_methods) = class_defs.definitions.get(&ref_.class_name) {
                if !class_methods.methods.contains(&ref_.method_name) {
                    let found_methods = if class_methods.methods.is_empty() {
                        "(none)".to_string()
                    } else {
                        class_methods
                            .methods
                            .iter()
                            .map(|m| m.value.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    };
                    self._report(
                        results,
                        &ref_.file,
                        &ref_.line,
                        "AES0303",
                        &format!(
                            "{} Method '{}' not found on class '{}'. Defined methods: {}.",
                            AES0303_CAPABILITY_ROUTING,
                            ref_.method_name.value,
                            ref_.class_name.value,
                            found_methods
                        ),
                    );
                }
            } else {
                self._report(
                    results,
                    &ref_.file,
                    &ref_.line,
                    "AES0303",
                    &format!(
                        "{} Class '{}' not found in any scanned file.",
                        AES0303_CAPABILITY_ROUTING, ref_.class_name.value
                    ),
                );
            }
        }
    }

    fn _check_single_capability_bottleneck(
        &self,
        capability_refs: &CapabilityReferenceList,
        class_defs: &ClassDefinitionMap,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
        _root_dir: &FilePath,
    ) {
        if capability_refs.references.is_empty() {
            return;
        }

        let class_usage = self._group_capabilities_by_class(capability_refs);

        if class_usage.usage.len() == 1 {
            let (single_class, usage_list) = match class_usage.usage.iter().next() {
                Some(x) => x,
                None => return,
            };
            if !usage_list.items.is_empty() {
                let other_classes: Vec<ClassNameVO> = class_defs
                    .definitions
                    .keys()
                    .filter(|c| *c != single_class)
                    .cloned()
                    .collect();
                if !other_classes.is_empty() && usage_list.items.len() >= 3 {
                    self._report_class_bottleneck(
                        results,
                        single_class,
                        usage_list,
                        &other_classes,
                    );
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
            class_usage
                .usage
                .entry(ref_.class_name.clone())
                .or_insert_with(|| ClassUsageItemList { items: vec![] })
                .items
                .push(ClassUsageItem {
                    file: ref_.file.clone(),
                    line: ref_.line.clone(),
                    method: ref_.method_name.clone(),
                });
        }
        class_usage
    }

    fn _report_class_bottleneck(
        &self,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
        class_name: &ClassNameVO,
        refs: &ClassUsageItemList,
        other_classes: &[ClassNameVO],
    ) {
        let other_names: Vec<String> = other_classes
            .iter()
            .take(5)
            .map(|c| c.value.clone())
            .collect();
        let other_str = other_names.join(", ");
        for item in &refs.items {
            self._report(
                results,
                &item.file,
                &item.line,
                "AES0303",
                &format!(
                    "{} Action '{}' routes to '{}' but {} other capability classes exist ({}).",
                    AES0303_SINGLE_BOTTLENECK,
                    item.method.value,
                    class_name.value,
                    other_classes.len(),
                    other_str
                ),
            );
        }
    }

    fn _check_missing_vo_construction(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &[FilePath],
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        for f in files {
            self._check_file_vo_construction(analyzer, f, results);
        }
    }

    fn _check_file_vo_construction(
        &self,
        _analyzer: &dyn IAnalyzer,
        file_path: &FilePath,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        let path = file_path.to_string();
        if !path.ends_with(".py") {
            return;
        }

        let content = match std::fs::read_to_string(path.as_str()) {
            Ok(c) => c,
            Err(_) => return,
        };

        let call_re = match Regex::new(r"(?:await\s+)?self\.\w+\.(\w+)\s*\(") {
            Ok(r) => r,
            Err(_) => return,
        };

        for caps in call_re.captures_iter(&content) {
            let method_name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let paren_start = caps.get(0).map(|m| m.end()).unwrap_or(1).saturating_sub(1);
            let args_vo = self._extract_args(&content, paren_start);
            if let Some(ref args_val) = args_vo.value {
                let args_text = args_val.trim();
                if args_text.is_empty() {
                    let line_no = content[..paren_start]
                        .chars()
                        .filter(|&c| c == '\n')
                        .count() as i64
                        + 1;
                    self._report(
                        results,
                        file_path,
                        &LineNumber::new(line_no),
                        "AES0303",
                        &format!(
                            "{} Capability call 'self.some_executor.{}()' missing required VO parameter.",
                            AES0303_MISSING_VO, method_name
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
            MethodArgsVO {
                value: Some(content[open_paren + 1..i - 1].to_string()),
            }
        } else {
            MethodArgsVO { value: None }
        }
    }

    fn _report(
        &self,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
        file: &FilePath,
        line: &LineNumber,
        code: &str,
        message: &str,
    ) {
        results.push(LintResult {
            file: file.clone(),
            line: line.clone(),
            column: ColumnNumber::new(1),
            code: ErrorCode::new(code).unwrap_or_else(|_| ErrorCode::raw(code)),
            message: LintMessage::new(message),
            severity: Severity::MEDIUM,
            source: Some(AdapterName::new("dispatch_routing").unwrap_or_default()),
            enclosing_scope: Some(crate::shared_common::taxonomy_lint_vo::ScopeRef::new("")),
            related_locations: crate::shared_common::taxonomy_lint_vo::LocationList::new(),
        });
    }
}

impl IDispatchRoutingProtocol for DispatchRoutingChecker {
    fn check_capability_routing(
        &self,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        todo!()
    }
}
