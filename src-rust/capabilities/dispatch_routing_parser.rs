// dispatch_routing_parser — Parser logic for capability class extraction.
// 1:1 Rust implementation matching capabilities/dispatch_routing_parser.py

use regex::Regex;
use once_cell::sync::Lazy;

use crate::taxonomy::{ClassDefinitionMap, ClassMethodsVO, ContentString};
use super::dispatch_parser_types::{BraceDepthVO, ClassParsingStateVO, ScopeNameVO};

pub struct DispatchRoutingParser;

impl DispatchRoutingParser {
    pub fn new() -> Self {
        Self
    }

    pub fn strip_docstrings(&self, text: &str) -> String {
        static DOUBLE_DOC_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#""""[\s\S]*?""""#).unwrap()
        });
        static SINGLE_DOC_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"'''[\s\S]*?'''").unwrap()
        });
        static COMMENT_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?m)^\s*#.*$").unwrap()
        });

        let mut result = DOUBLE_DOC_RE.replace_all(text, "").to_string();
        result = SINGLE_DOC_RE.replace_all(&result, "").to_string();
        result = COMMENT_RE.replace_all(&result, "").to_string();
        result
    }

    pub fn extract_class_methods(&self, text: &str) -> ClassDefinitionMap {
        let mut result = ClassDefinitionMap {
            definitions: std::collections::HashMap::new(),
        };
        let mut current_class = ScopeNameVO::default();
        let mut class_brace_depth = BraceDepthVO::default();
        let mut current_brace_depth = BraceDepthVO::default();

        for line in text.lines() {
            let stripped = line.trim();
            let state = self._process_class_line(
                stripped,
                line,
                ClassParsingStateVO {
                    current_class: current_class.clone(),
                    class_brace_depth: class_brace_depth.clone(),
                    current_brace_depth: current_brace_depth.clone(),
                },
                &mut result,
            );
            current_class = state.current_class;
            class_brace_depth = state.class_brace_depth;
            current_brace_depth = state.current_brace_depth;
        }

        result
    }

    fn _process_class_line(
        &self,
        stripped: &str,
        line: &str,
        mut state: ClassParsingStateVO,
        result: &mut ClassDefinitionMap,
    ) -> ClassParsingStateVO {
        let class_result = self._handle_class_definition(
            stripped,
            &state.current_class,
            result,
            &state.class_brace_depth,
            &state.current_brace_depth,
        );
        if let Some(new_state) = class_result {
            return new_state;
        }

        if state.current_class.value.is_some() {
            self._handle_method_definition(stripped, line, &state.current_class, result);

            let open = stripped.chars().filter(|&c| c == '{').count();
            let close = stripped.chars().filter(|&c| c == '}').count();
            state.current_brace_depth.value += open as i32 - close as i32;

            state.current_class = self._handle_scope_exit(
                line,
                &state.current_class,
                &state.class_brace_depth,
                &state.current_brace_depth,
            );
        }

        ClassParsingStateVO {
            current_class: state.current_class,
            class_brace_depth: state.class_brace_depth,
            current_brace_depth: state.current_brace_depth,
        }
    }

    fn _handle_class_definition(
        &self,
        stripped: &str,
        current_class: &ScopeNameVO,
        result: &mut ClassDefinitionMap,
        _class_brace_depth: &BraceDepthVO,
        current_brace_depth: &BraceDepthVO,
    ) -> Option<ClassParsingStateVO> {
        static CLASS_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^class\s+([A-Za-z_][\w]*)\s*[\(:]").unwrap()
        });

        let caps = CLASS_RE.captures(stripped)?;
        let new_class_name = caps.get(1).unwrap().as_str().to_string();
        result.definitions.insert(new_class_name.clone(), ClassMethodsVO { methods: vec![] });

        let mut new_brace = current_brace_depth.clone();
        new_brace.value += stripped.chars().filter(|&c| c == '{').count() as i32
            - stripped.chars().filter(|&c| c == '}').count() as i32;

        Some(ClassParsingStateVO {
            current_class: ScopeNameVO { value: Some(new_class_name) },
            class_brace_depth: BraceDepthVO { value: current_brace_depth.value },
            current_brace_depth: new_brace,
        })
    }

    fn _handle_method_definition(
        &self,
        stripped: &str,
        line: &str,
        current_class: &ScopeNameVO,
        result: &mut ClassDefinitionMap,
    ) {
        static METHOD_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?:async\s+)?def\s+([A-Za-z_][\w]*)\s*\(").unwrap()
        });

        if let Some(caps) = METHOD_RE.captures(stripped) {
            if let Some(ref class_name) = current_class.value {
                let indent = line.len() - line.trim_start().len();
                if indent <= 8 {
                    if let Some(methods_vo) = result.definitions.get_mut(class_name) {
                        methods_vo.methods.push(caps.get(1).unwrap().as_str().to_string());
                    }
                }
            }
        }
    }

    fn _handle_scope_exit(
        &self,
        line: &str,
        current_class: &ScopeNameVO,
        class_brace_depth: &BraceDepthVO,
        current_brace_depth: &BraceDepthVO,
    ) -> ScopeNameVO {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return current_class.clone();
        }

        let is_indented = line.starts_with(' ') || line.starts_with('\t');

        if !is_indented && current_brace_depth.value <= class_brace_depth.value {
            if !trimmed.starts_with('@')
                && !trimmed.starts_with("class ")
                && !trimmed.starts_with("def ")
                && !trimmed.starts_with("async ")
            {
                return ScopeNameVO { value: None };
            }
        }
        current_class.clone()
    }
}
