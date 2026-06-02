use std::collections::HashSet;
use std::fs;
use std::path::Path;
use regex::Regex;

use crate::taxonomy::{
    ArchitectureConfig, LayerDefinition,
    Severity, ErrorCode, FilePath, LineNumber,
    ColumnNumber, LayerNameVO,
    LintResult, LintMessage, AdapterName, ScopeRef, LocationList,
};
use crate::infrastructure::ast_py_scanner::ASTPythonParserAdapter;

pub struct ArchitectureRulesEvaluator {
    config: ArchitectureConfig,
}

impl ArchitectureRulesEvaluator {
    pub fn new(config_json: &str) -> Result<Self, String> {
        let config: ArchitectureConfig = serde_json::from_str(config_json)
            .map_err(|e| format!("Failed to parse ArchitectureConfig JSON: {}", e))?;
        Ok(Self { config })
    }

    pub fn evaluate_all(&self, files: &[String], root_dir: &str) -> Vec<LintResult> {
        let mut violations = Vec::new();

        for file in files {
            let path = Path::new(file);
            let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
            if filename.is_empty() {
                continue;
            }

            let layer_name = self.detect_layer(file, root_dir);
            let definition = layer_name.as_ref().and_then(|name| {
                let key = LayerNameVO::new(name);
                self.config.layers.get(&key)
            });

            // A. File Naming check (AES003)
            self.check_file_naming(file, filename, &layer_name, definition, &mut violations);

            // B. Domain Suffixes check (AES010/AES011)
            self.check_domain_suffixes(file, filename, definition, &mut violations);

            // C. Internal rules check (AES012/AES013/AES006)
            self.check_internal_rules(file, filename, &layer_name, definition, &mut violations);

            // D. Metric check (AES004/AES005/AES009)
            self.check_metrics(file, filename, &layer_name, definition, &mut violations);

            // E. Role check (AES021/AES022/AES023/AES024)
            self.check_roles(file, filename, &layer_name, definition, &mut violations);

            // F. Import check (AES001/AES002)
            self.check_imports(file, filename, &layer_name, definition, root_dir, &mut violations);
        }

        violations
    }

    fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String> {
        let normalized_path = file_path.replace('\\', "/");
        let normalized_root = root_dir.replace('\\', "/");

        let rel_path = if normalized_path.starts_with(&normalized_root) {
            let mut r = &normalized_path[normalized_root.len()..];
            if r.starts_with('/') {
                r = &r[1..];
            }
            r.to_string()
        } else {
            normalized_path.clone()
        };

        // Sort layers by path length descending to match most specific first
        let mut sorted_layers: Vec<(&LayerNameVO, &LayerDefinition)> = self.config.layers.iter().collect();
        sorted_layers.sort_by_key(|(_, def)| std::cmp::Reverse(def.path.value.len()));

        for (name, def) in sorted_layers {
            let path_def = &def.path.value;
            if rel_path.starts_with(path_def) || rel_path.contains(&format!("/{}/", path_def)) {
                return Some(name.value.clone());
            }
        }
        None
    }

    fn detect_module_layer(&self, module_path: &str) -> Option<String> {
        let parts: Vec<&str> = module_path.split('.').collect();
        for part in parts {
            for (name, def) in &self.config.layers {
                let path_last = def.path.value.split('/').last().unwrap_or("");
                if part == name.value || part == path_last {
                    return Some(name.value.clone());
                }
            }
        }
        None
    }

    fn is_barrel_file(&self, filename: &str) -> bool {
        filename == "__init__.py" || filename == "mod.rs" || filename == "index.ts" || filename == "index.js"
    }

    fn is_entry_point(&self, filename: &str) -> bool {
        filename == "__init__.py" || filename == "main.py" || filename == "py.typed" || filename == "app.py" || filename == "lib.rs"
    }

    fn check_file_naming(
        &self,
        file: &str,
        filename: &str,
        _layer_name: &Option<String>,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    ) {
        if self.is_barrel_file(filename) || self.is_entry_point(filename) {
            return;
        }

        // Exemptions check
        if let Some(def) = definition {
            if def.exceptions.values.contains(&filename.to_string()) {
                return;
            }
        }

        let expected_word_count = if let Some(def) = definition {
            if def.word_count.value > 0 {
                def.word_count.value as i32
            } else {
                self.config.naming.word_count.value as i32
            }
        } else {
            self.config.naming.word_count.value as i32
        };

        let stem = filename.split('.').next().unwrap_or("");
        
        // Match expected snake_case words separated by underscores
        let naming_regex = format!(r"^[a-z0-9]+(_[a-z0-9]+){{{}}}$", expected_word_count - 1);
        if let Ok(re) = Regex::new(&naming_regex) {
            if !re.is_match(stem) {
                let violation_msg = if let Some(def) = definition {
                    if !def.word_count_violation_message.value.is_empty() {
                        def.word_count_violation_message.value.clone()
                    } else if !self.config.naming.word_count_violation_message.value.is_empty() {
                        self.config.naming.word_count_violation_message.value.clone()
                    } else {
                        format!(
                            "AES003 NAMING_CONVENTION: Filename does not follow the {}-word underscore-separated pattern.\n\
                            WHY? Strict three-word names ensure architectural consistency and prevent naming ambiguity.\n\
                            FIX: Rename the file to exactly {} words separated by underscores (e.g., word1_word2_word3.py).",
                            expected_word_count, expected_word_count
                        )
                    }
                } else if !self.config.naming.word_count_violation_message.value.is_empty() {
                    self.config.naming.word_count_violation_message.value.clone()
                } else {
                    format!(
                        "AES003 NAMING_CONVENTION: Filename does not follow the {}-word underscore-separated pattern.\n\
                        WHY? Strict three-word names ensure architectural consistency and prevent naming ambiguity.\n\
                        FIX: Rename the file to exactly {} words separated by underscores (e.g., word1_word2_word3.py).",
                        expected_word_count, expected_word_count
                    )
                };

                violations.push(LintResult {
                    file: FilePath::new(file.to_string()),
                    line: LineNumber::new(0),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::new("AES003"),
                    message: LintMessage::new(violation_msg),
                    source: AdapterName::new("architecture"),
                    severity: Severity::HIGH,
                    enclosing_scope: ScopeRef {
                        name: "".to_string(),
                        kind: "".to_string(),
                        file: FilePath::new(""),
                        start_line: LineNumber::new(0),
                        end_line: LineNumber::new(0),
                    },
                    related_locations: LocationList::new(Vec::new()),
                });
            }
        }
    }

    fn check_domain_suffixes(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    ) {
        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if self.is_barrel_file(filename) || self.is_entry_point(filename) {
            return;
        }

        if def.exceptions.values.contains(&filename.to_string()) {
            return;
        }

        let stem = filename.split('.').next().unwrap_or("");
        let parts: Vec<&str> = stem.split('_').collect();
        let suffix = if parts.len() >= 2 { Some(*parts.last().unwrap()) } else { None };

        // 1. Forbidden suffix check
        if let Some(s) = suffix {
            if def.forbidden_suffix.values.contains(&s.to_string()) {
                let msg = if !def.suffix_violation_message.value.is_empty() {
                    def.suffix_violation_message.value.clone()
                } else {
                    "AES011 SUFFIX_MISMATCH: File uses a forbidden suffix for this layer.\n\
                    WHY? Forbidden suffixes prevent technical concepts from leaking into domain layers.\n\
                    FIX: Rename the file to use an allowed suffix or move it to the correct layer.".to_string()
                };
                violations.push(LintResult {
                    file: FilePath::new(file.to_string()),
                    line: LineNumber::new(0),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::new("AES010"),
                    message: LintMessage::new(msg),
                    source: AdapterName::new("architecture"),
                    severity: Severity::HIGH,
                    enclosing_scope: ScopeRef {
                        name: "".to_string(),
                        kind: "".to_string(),
                        file: FilePath::new(""),
                        start_line: LineNumber::new(0),
                        end_line: LineNumber::new(0),
                    },
                    related_locations: LocationList::new(Vec::new()),
                });
                return;
            }
        }

        // 2. Strict suffix policy check
        if def.suffix_policy.value == "strict" {
            let matches_allowed = suffix.map(|s| def.allowed_suffix.values.contains(&s.to_string())).unwrap_or(false);
            if !matches_allowed {
                let msg = if !def.suffix_violation_message.value.is_empty() {
                    def.suffix_violation_message.value.clone()
                } else {
                    format!(
                        "AES011 SUFFIX_MISMATCH: File is missing a required strict suffix for this layer.\n\
                        WHY? Strict suffixes ensure that every component in this layer has a clear, standardized role.\n\
                        FIX: Add one of the required suffixes: {}.",
                        def.allowed_suffix.values.join(", ")
                    )
                };
                violations.push(LintResult {
                    file: FilePath::new(file.to_string()),
                    line: LineNumber::new(0),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::new("AES011"),
                    message: LintMessage::new(msg),
                    source: AdapterName::new("architecture"),
                    severity: Severity::HIGH,
                    enclosing_scope: ScopeRef {
                        name: "".to_string(),
                        kind: "".to_string(),
                        file: FilePath::new(""),
                        start_line: LineNumber::new(0),
                        end_line: LineNumber::new(0),
                    },
                    related_locations: LocationList::new(Vec::new()),
                });
            }
        }
    }

    fn check_internal_rules(
        &self,
        file: &str,
        filename: &str,
        _layer_name: &Option<String>,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    ) {
        let def = match definition {
            Some(d) => d,
            None => return,
        };

        let scanner = ASTPythonParserAdapter::new();
        let filepath = crate::taxonomy::FilePath::new(file.to_string());

        if self.is_barrel_file(filename) {
            // Check barrel completeness
            if def.barrel_completeness.value {
                let has_export = scanner.has_all_export(&filepath).value.value;
                if !has_export {
                    let msg = if !def.barrel_completeness_violation_message.value.is_empty() {
                        def.barrel_completeness_violation_message.value.clone()
                    } else {
                        "__init__.py missing __all__ export list.".to_string()
                    };
                    violations.push(LintResult {
                        file: FilePath::new(file.to_string()),
                        line: LineNumber::new(0),
                        column: ColumnNumber::new(0),
                        code: ErrorCode::new("AES012"),
                        message: LintMessage::new(msg),
                        source: AdapterName::new("architecture"),
                        severity: Severity::MEDIUM,
                        enclosing_scope: ScopeRef {
                            name: "".to_string(),
                            kind: "".to_string(),
                            file: FilePath::new(""),
                            start_line: LineNumber::new(0),
                            end_line: LineNumber::new(0),
                        },
                        related_locations: LocationList::new(Vec::new()),
                    });
                }
            }
        } else {
            // Check forbid internal __all__ in non-barrel files
            if def.forbid_internal_all.value {
                let has_export = scanner.has_all_export(&filepath).value.value;
                if has_export {
                    let msg = if !def.forbid_internal_all_violation_message.value.is_empty() {
                        def.forbid_internal_all_violation_message.value.clone()
                    } else {
                        "__all__ is forbidden in non-barrel files.".to_string()
                    };
                    violations.push(LintResult {
                        file: FilePath::new(file.to_string()),
                        line: LineNumber::new(0),
                        column: ColumnNumber::new(0),
                        code: ErrorCode::new("AES013"),
                        message: LintMessage::new(msg),
                        source: AdapterName::new("architecture"),
                        severity: Severity::MEDIUM,
                        enclosing_scope: ScopeRef {
                            name: "".to_string(),
                            kind: "".to_string(),
                            file: FilePath::new(""),
                            start_line: LineNumber::new(0),
                            end_line: LineNumber::new(0),
                        },
                        related_locations: LocationList::new(Vec::new()),
                    });
                }
            }

            // Check no primitives
            if def.no_primitives.value {
                let target_primitives: Vec<String> = crate::taxonomy::PRIMITIVE_TYPE_LIST()
                    .values
                    .iter()
                    .map(|s| s.value.clone())
                    .collect();

                let primitive_list = crate::taxonomy::PrimitiveTypeList::new(
                    target_primitives.into_iter().map(crate::taxonomy::SymbolName::new).collect()
                );
                let prim_violations = scanner.find_primitive_violations(&filepath, &primitive_list);
                for pv in prim_violations.values {
                    let msg = if !def.no_primitives_violation_message.value.is_empty() {
                        def.no_primitives_violation_message.value.clone()
                    } else {
                        "Use Value Objects instead of raw primitives.".to_string()
                    };
                    violations.push(LintResult {
                        file: FilePath::new(file.to_string()),
                        line: LineNumber::new(pv.line.value),
                        column: ColumnNumber::new(pv.column.value),
                        code: ErrorCode::new("AES006"),
                        message: LintMessage::new(msg),
                        source: AdapterName::new("architecture"),
                        severity: Severity::MEDIUM,
                        enclosing_scope: ScopeRef {
                            name: "".to_string(),
                            kind: "".to_string(),
                            file: FilePath::new(""),
                            start_line: LineNumber::new(0),
                            end_line: LineNumber::new(0),
                        },
                        related_locations: LocationList::new(Vec::new()),
                    });
                }
            }
        }
    }

    fn check_metrics(
        &self,
        file: &str,
        filename: &str,
        _layer_name: &Option<String>,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    ) {
        if filename == "__init__.py" {
            return;
        }

        // Exemptions check
        if let Some(def) = definition {
            if def.exceptions.values.contains(&filename.to_string()) {
                return;
            }
        }

        // Get file line count
        let content = fs::read_to_string(file).unwrap_or_default();
        let line_count = content.lines().count() as i32;

        if let Some(def) = definition {
            // 1. Min lines
            let min = def.min_lines.value as i32;
            if min > 0 && line_count < min {
                let msg = if !def.min_lines_violation_message.value.is_empty() {
                    def.min_lines_violation_message.value.clone()
                } else {
                    format!(
                        "AES005 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
                        WHY? Excessively small files clutter the project structure; logic should be merged into a parent module.\n\
                        FIX: Expand the component or merge this logic into a related module (min: {}).",
                        min
                    )
                };
                violations.push(LintResult {
                    file: FilePath::new(file.to_string()),
                    line: LineNumber::new(0),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::new("AES005"),
                    message: LintMessage::new(msg),
                    source: AdapterName::new("architecture"),
                    severity: Severity::HIGH,
                    enclosing_scope: ScopeRef {
                        name: "".to_string(),
                        kind: "".to_string(),
                        file: FilePath::new(""),
                        start_line: LineNumber::new(0),
                        end_line: LineNumber::new(0),
                    },
                    related_locations: LocationList::new(Vec::new()),
                });
            }

            // 2. Max lines
            let max = def.max_lines.value as i32;
            if max > 0 && line_count > max {
                let msg = if !def.max_lines_violation_message.value.is_empty() {
                    def.max_lines_violation_message.value.clone()
                } else {
                    format!(
                        "AES004 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                        WHY? Large files violate the Single Responsibility Principle and are difficult to maintain or test.\n\
                        FIX: Split the module into smaller, more focused files (max: {}).",
                        max
                    )
                };
                violations.push(LintResult {
                    file: FilePath::new(file.to_string()),
                    line: LineNumber::new(0),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::new("AES004"),
                    message: LintMessage::new(msg),
                    source: AdapterName::new("architecture"),
                    severity: Severity::HIGH,
                    enclosing_scope: ScopeRef {
                        name: "".to_string(),
                        kind: "".to_string(),
                        file: FilePath::new(""),
                        start_line: LineNumber::new(0),
                        end_line: LineNumber::new(0),
                    },
                    related_locations: LocationList::new(Vec::new()),
                });
            }
        }

        // 3. Mandatory Class Definition
        let mut check_mandatory_class = false;
        let mut mandatory_class_msg = None;
        if let Some(def) = definition {
            if def.mandatory_class_definition.value {
                check_mandatory_class = true;
                mandatory_class_msg = Some(def.mandatory_class_definition_violation_message.value.clone());
            }
        } else if self.config.mandatory_class_definition.value {
            check_mandatory_class = true;
            mandatory_class_msg = Some(self.config.mandatory_class_definition_violation_message.value.clone());
        }

        if check_mandatory_class {
            if filename != "__init__.py" && filename != "main.py" && filename != "py.typed" {
                let scanner = ASTPythonParserAdapter::new();
                let filepath = crate::taxonomy::FilePath::new(file.to_string());
                if let Ok(class_meta) = scanner.get_class_definitions(&filepath) {
                    let classes = class_meta.value.get("classes").and_then(|c| c.as_array());
                    let has_class = classes.map(|arr| !arr.is_empty()).unwrap_or(false);
                    if !has_class {
                        let msg = if let Some(ref m) = mandatory_class_msg {
                            if !m.is_empty() {
                                m.clone()
                            } else {
                                "AES009 MANDATORY_CLASS_DEFINITION: File is missing a class definition.\n\
                                WHY? Encapsulation in classes is required for proper dependency injection and contract adherence.\n\
                                FIX: Move standalone functions into a class that implements its corresponding domain contract.".to_string()
                            }
                        } else {
                            "AES009 MANDATORY_CLASS_DEFINITION: File is missing a class definition.\n\
                            WHY? Encapsulation in classes is required for proper dependency injection and contract adherence.\n\
                            FIX: Move standalone functions into a class that implements its corresponding domain contract.".to_string()
                        };
                        violations.push(LintResult {
                            file: FilePath::new(file.to_string()),
                            line: LineNumber::new(0),
                            column: ColumnNumber::new(0),
                            code: ErrorCode::new("AES009"),
                            message: LintMessage::new(msg),
                            source: AdapterName::new("architecture"),
                            severity: Severity::HIGH,
                            enclosing_scope: ScopeRef {
                                name: "".to_string(),
                                kind: "".to_string(),
                                file: FilePath::new(""),
                                start_line: LineNumber::new(0),
                                end_line: LineNumber::new(0),
                            },
                            related_locations: LocationList::new(Vec::new()),
                        });
                    }
                }
            }
        }
    }

    fn check_roles(
        &self,
        file: &str,
        filename: &str,
        layer_name: &Option<String>,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    ) {
        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if def.exceptions.values.contains(&filename.to_string()) {
            return;
        }

        let is_agent = layer_name.as_ref().map(|l| l == "agent" || l.starts_with("agent(")).unwrap_or(false);
        let is_surface = layer_name.as_ref().map(|l| l == "surfaces" || l.starts_with("surfaces(")).unwrap_or(false);

        let scanner = ASTPythonParserAdapter::new();
        let filepath = crate::taxonomy::FilePath::new(file.to_string());

        if is_agent {
            // 1. Stateless Execution
            if def.stateless_execution.value {
                let assigns = scanner.get_assignment_targets(&filepath).value.get("assignments")
                    .and_then(|a| a.as_array()).cloned().unwrap_or_default();
                let methods_meta = scanner.get_class_methods(&filepath);
                
                // Parse class methods structure: methods inside a "methods" key
                if let Some(methods_val) = methods_meta.value.get("methods").and_then(|m| m.as_object()) {
                    for assign in assigns {
                        let line_val = assign.get("line").and_then(|l| l.as_i64()).unwrap_or(0);
                        let mut outside_init = false;
                        
                        // Find if this assignment is in a method other than __init__
                        for (_class_name, class_methods_val) in methods_val {
                            if let Some(methods_arr) = class_methods_val.as_array() {
                                for m in methods_arr {
                                    if let Some(m_obj) = m.as_object() {
                                        let name = m_obj.get("name").and_then(|n| n.as_str()).unwrap_or("");
                                        let m_line = m_obj.get("line").and_then(|l| l.as_i64()).unwrap_or(0);
                                        if name != "__init__" && line_val >= m_line {
                                            outside_init = true;
                                        }
                                    }
                                }
                            }
                        }
                        
                        if outside_init {
                            let msg = if def.stateless_execution_violation_message.value.is_empty() {
                                "Non-stateless behavior detected: state assignment found outside __init__.".to_string()
                            } else {
                                def.stateless_execution_violation_message.value.clone()
                            };
                            violations.push(LintResult {
                                file: FilePath::new(file.to_string()),
                                line: LineNumber::new(line_val),
                                column: ColumnNumber::new(0),
                                code: ErrorCode::new("AES021"),
                                message: LintMessage::new(msg),
                                source: AdapterName::new("architecture"),
                                severity: Severity::HIGH,
                                enclosing_scope: ScopeRef {
                                    name: "".to_string(),
                                    kind: "".to_string(),
                                    file: FilePath::new(""),
                                    start_line: LineNumber::new(0),
                                    end_line: LineNumber::new(0),
                                },
                                related_locations: LocationList::new(Vec::new()),
                            });
                        }
                    }
                }
            }

            // 2. High Level Policy Only
            if def.high_level_policy_only.value {
                if let Ok(imports) = scanner.extract_imports(&filepath) {
                    for imp in imports.values {
                        let mod_str = imp.module.value.clone();
                        if mod_str.contains("infrastructure") {
                            let msg = if def.high_level_policy_only_violation_message.value.is_empty() {
                                "Low-level implementation details found (infrastructure import).".to_string()
                            } else {
                                def.high_level_policy_only_violation_message.value.clone()
                            };
                            violations.push(LintResult {
                                file: FilePath::new(file.to_string()),
                                line: LineNumber::new(imp.line.value),
                                column: ColumnNumber::new(0),
                                code: ErrorCode::new("AES021"),
                                message: LintMessage::new(msg),
                                source: AdapterName::new("architecture"),
                                severity: Severity::HIGH,
                                enclosing_scope: ScopeRef {
                                    name: "".to_string(),
                                    kind: "".to_string(),
                                    file: FilePath::new(""),
                                    start_line: LineNumber::new(0),
                                    end_line: LineNumber::new(0),
                                },
                                related_locations: LocationList::new(Vec::new()),
                            });
                        }
                    }
                }
            }

            // 3. Coordinates Multiple Orchestrators
            if def.coordinates_multiple_orchestrators.value {
                if let Ok(class_meta) = scanner.get_class_definitions(&filepath) {
                    if let Some(classes) = class_meta.value.get("classes").and_then(|c| c.as_array()) {
                        let methods_meta = scanner.get_class_methods(&filepath);
                        if let Some(methods_val) = methods_meta.value.get("methods").and_then(|m| m.as_object()) {
                            for cls in classes {
                                let cname = cls.get("name").and_then(|n| n.as_str()).unwrap_or("");
                                if let Some(class_methods_val) = methods_val.get(cname).and_then(|m| m.as_array()) {
                                    for m in class_methods_val {
                                        if let Some(m_obj) = m.as_object() {
                                            let name = m_obj.get("name").and_then(|n| n.as_str()).unwrap_or("");
                                            if name == "__init__" {
                                                let args = m_obj.get("args").and_then(|a| a.as_array()).cloned().unwrap_or_default();
                                                let mut orchestrator_count = 0;
                                                for arg in args {
                                                    if let Some(arg_str) = arg.as_str() {
                                                        if arg_str.to_lowercase().contains("orchestrator") {
                                                            orchestrator_count += 1;
                                                        }
                                                    }
                                                }
                                                if orchestrator_count < 2 {
                                                    let line_val = m_obj.get("line").and_then(|l| l.as_i64()).unwrap_or(0);
                                                    let msg = if def.coordinates_multiple_orchestrators_violation_message.value.is_empty() {
                                                        "Coordinator must manage multiple orchestrators.".to_string()
                                                    } else {
                                                        def.coordinates_multiple_orchestrators_violation_message.value.clone()
                                                    };
                                                    violations.push(LintResult {
                                                        file: FilePath::new(file.to_string()),
                                                        line: LineNumber::new(line_val),
                                                        column: ColumnNumber::new(0),
                                                        code: ErrorCode::new("AES021"),
                                                        message: LintMessage::new(msg),
                                                        source: AdapterName::new("architecture"),
                                                        severity: Severity::MEDIUM,
                                                        enclosing_scope: ScopeRef {
                                                            name: "".to_string(),
                                                            kind: "".to_string(),
                                                            file: FilePath::new(""),
                                                            start_line: LineNumber::new(0),
                                                            end_line: LineNumber::new(0),
                                                        },
                                                        related_locations: LocationList::new(Vec::new()),
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 4. Must Implement ServiceContainerAggregate
            if def.must_implement_service_container_aggregate.value {
                if let Ok(class_meta) = scanner.get_class_definitions(&filepath) {
                    if let Some(classes) = class_meta.value.get("classes").and_then(|c| c.as_array()) {
                        for cls in classes {
                            let resolved_bases = cls.get("resolved_bases").and_then(|b| b.as_array()).cloned().unwrap_or_default();
                            let bases = cls.get("bases").and_then(|b| b.as_array()).cloned().unwrap_or_default();
                            let mut implements_container = false;
                            
                            for base in resolved_bases.iter().chain(bases.iter()) {
                                if let Some(base_str) = base.as_str() {
                                    if base_str.contains("ServiceContainerAggregate") {
                                        implements_container = true;
                                    }
                                }
                            }

                            if !implements_container {
                                let line_val = cls.get("line").and_then(|l| l.as_i64()).unwrap_or(0);
                                let msg = if def.must_implement_service_container_aggregate_violation_message.value.is_empty() {
                                    "Class must implement ServiceContainerAggregate.".to_string()
                                } else {
                                    def.must_implement_service_container_aggregate_violation_message.value.clone()
                                };
                                violations.push(LintResult {
                                    file: FilePath::new(file.to_string()),
                                    line: LineNumber::new(line_val),
                                    column: ColumnNumber::new(0),
                                    code: ErrorCode::new("AES021"),
                                    message: LintMessage::new(msg),
                                    source: AdapterName::new("architecture"),
                                    severity: Severity::HIGH,
                                    enclosing_scope: ScopeRef {
                                        name: "".to_string(),
                                        kind: "".to_string(),
                                        file: FilePath::new(""),
                                        start_line: LineNumber::new(0),
                                        end_line: LineNumber::new(0),
                                    },
                                    related_locations: LocationList::new(Vec::new()),
                                });
                            }
                        }
                    }
                }
            }

            // 5. Lazy Eager Initialization Only
            if def.lazy_eager_initialization_only.value {
                let control_flow = scanner.get_control_flow_count(&filepath).value;
                if control_flow > 2 {
                    let msg = if def.lazy_eager_initialization_only_violation_message.value.is_empty() {
                        "Complex initialization logic found in Container.".to_string()
                    } else {
                        def.lazy_eager_initialization_only_violation_message.value.clone()
                    };
                    violations.push(LintResult {
                        file: FilePath::new(file.to_string()),
                        line: LineNumber::new(0),
                        column: ColumnNumber::new(0),
                        code: ErrorCode::new("AES021"),
                        message: LintMessage::new(msg),
                        source: AdapterName::new("architecture"),
                        severity: Severity::HIGH,
                        enclosing_scope: ScopeRef {
                            name: "".to_string(),
                            kind: "".to_string(),
                            file: FilePath::new(""),
                            start_line: LineNumber::new(0),
                            end_line: LineNumber::new(0),
                        },
                        related_locations: LocationList::new(Vec::new()),
                    });
                }
            }

            // 6. Forbid Any Type (AES024)
            if def.forbid_any_type.value {
                // In Rust, we can quickly search lines for any `Any` annotations using regex
                let any_annotation_regex = Regex::new(r":\s*Any\b|->\s*Any\b|:\s*typing\.Any\b|->\s*typing\.Any\b").unwrap();
                let content = fs::read_to_string(file).unwrap_or_default();
                for (idx, line) in content.lines().enumerate() {
                    let stripped = line.trim();
                    if stripped.starts_with('#') || stripped.starts_with("import ") || stripped.starts_with("from ") {
                        continue;
                    }
                    if any_annotation_regex.is_match(stripped) {
                        let msg = if def.forbid_any_type_violation_message.value.is_empty() {
                            format!("`Any` type annotation found in agent orchestrator layer: '{}'.", stripped)
                        } else {
                            def.forbid_any_type_violation_message.value.clone()
                        };
                        violations.push(LintResult {
                            file: FilePath::new(file.to_string()),
                            line: LineNumber::new((idx + 1) as i64),
                            column: ColumnNumber::new(0),
                            code: ErrorCode::new("AES024"),
                            message: LintMessage::new(msg),
                            source: AdapterName::new("architecture"),
                            severity: Severity::HIGH,
                            enclosing_scope: ScopeRef {
                                name: "".to_string(),
                                kind: "".to_string(),
                                file: FilePath::new(""),
                                start_line: LineNumber::new(0),
                                end_line: LineNumber::new(0),
                            },
                            related_locations: LocationList::new(Vec::new()),
                        });
                    }
                }
            }
        }

        if is_surface {
            // Surfaces should not have domain logic
            if def.no_domain_logic.value {
                let control_flow = scanner.get_control_flow_count(&filepath).value;
                if control_flow > 3 {
                    let msg = if def.no_domain_logic_violation_message.value.is_empty() {
                        "Complex domain logic detected in a passive layer/role.".to_string()
                    } else {
                        def.no_domain_logic_violation_message.value.clone()
                    };
                    violations.push(LintResult {
                        file: FilePath::new(file.to_string()),
                        line: LineNumber::new(0),
                        column: ColumnNumber::new(0),
                        code: ErrorCode::new("AES022"),
                        message: LintMessage::new(msg),
                        source: AdapterName::new("architecture"),
                        severity: Severity::HIGH,
                        enclosing_scope: ScopeRef {
                            name: "".to_string(),
                            kind: "".to_string(),
                            file: FilePath::new(""),
                            start_line: LineNumber::new(0),
                            end_line: LineNumber::new(0),
                        },
                        related_locations: LocationList::new(Vec::new()),
                    });
                }
            }

            // Forbidden / allowed imports for surface layer
            if let Ok(imports) = scanner.extract_imports(&filepath) {
                let mut has_contract_import = false;
                let mut has_agent_import = false;

                for imp in imports.values {
                    let mod_str = imp.module.value.clone();
                    if mod_str == "contract" || mod_str.starts_with("contract.") {
                        has_contract_import = true;
                    }
                    if mod_str == "agent" || mod_str.starts_with("agent.") {
                        has_agent_import = true;
                    }

                    // Surface strict check: cannot import from infra, capabilities, etc.
                    let target_layer = self.detect_module_layer(&mod_str);
                    if let Some(ref tl) = target_layer {
                        if tl != "contract" && tl != "taxonomy" && tl != "surfaces" && tl != "agent" {
                            violations.push(LintResult {
                                file: FilePath::new(file.to_string()),
                                line: LineNumber::new(imp.line.value),
                                column: ColumnNumber::new(0),
                                code: ErrorCode::new("AES023"),
                                message: LintMessage::new(format!(
                                    "SURFACE DEPENDENCY VIOLATION: Surface layer is only allowed to import from 'contract' and 'taxonomy'. Found import from '{}'.",
                                    tl
                                )),
                                source: AdapterName::new("architecture"),
                                severity: Severity::HIGH,
                                enclosing_scope: ScopeRef {
                                    name: "".to_string(),
                                    kind: "".to_string(),
                                    file: FilePath::new(""),
                                    start_line: LineNumber::new(0),
                                    end_line: LineNumber::new(0),
                                },
                                related_locations: LocationList::new(Vec::new()),
                            });
                        }
                    }
                }

                if has_agent_import && !has_contract_import {
                    violations.push(LintResult {
                        file: FilePath::new(file.to_string()),
                        line: LineNumber::new(0),
                        column: ColumnNumber::new(0),
                        code: ErrorCode::new("AES023"),
                        message: LintMessage::new("AGENT MANDATORY IMPORT: Agent-related layer must import from 'contract'."),
                        source: AdapterName::new("architecture"),
                        severity: Severity::MEDIUM,
                        enclosing_scope: ScopeRef {
                            name: "".to_string(),
                            kind: "".to_string(),
                            file: FilePath::new(""),
                            start_line: LineNumber::new(0),
                            end_line: LineNumber::new(0),
                        },
                        related_locations: LocationList::new(Vec::new()),
                    });
                }
            }
        }
    }

    fn check_imports(
        &self,
        file: &str,
        filename: &str,
        layer_name: &Option<String>,
        definition: Option<&LayerDefinition>,
        _root_dir: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if self.is_barrel_file(filename) {
            return;
        }

        if def.exceptions.values.contains(&filename.to_string()) {
            return;
        }

        let scanner = ASTPythonParserAdapter::new();
        let filepath = crate::taxonomy::FilePath::new(file.to_string());

        // 1. Mandatory imports check
        let mut missing_mandatory = vec![];
        if !def.mandatory_import.values.is_empty() || !def.mandatory_imports.is_empty() {
            if let Ok(raw_symbols) = scanner.get_raw_symbols(&filepath) {
                let aliases = raw_symbols.value.get("aliases").and_then(|a| a.as_object()).cloned().unwrap_or_default();
                let used = raw_symbols.value.get("used").and_then(|u| u.as_array()).cloned().unwrap_or_default();
                let class_bases = raw_symbols.value.get("class_bases").and_then(|cb| cb.as_object()).cloned().unwrap_or_default();

                let mut real_usages = HashSet::new();
                for u in used {
                    if let Some(u_str) = u.as_str() {
                        // Check if it is a bypass marker
                        if !u_str.starts_with("_arch_") && u_str != "_" {
                            real_usages.insert(u_str.to_string());
                        }
                    }
                }

                let mut all_bases = HashSet::new();
                for (_, base_list) in class_bases {
                    if let Some(arr) = base_list.as_array() {
                        for b in arr {
                            if let Some(b_str) = b.as_str() {
                                all_bases.insert(b_str.to_string());
                            }
                        }
                    }
                }

                // Check general mandatory imports
                for req_layer in &def.mandatory_import.values {
                    let mut satisfied = false;
                    
                    // Specific logic for contract imports
                    if req_layer.starts_with("contract") {
                        for (alias, fullname) in &aliases {
                            if let Some(full_str) = fullname.as_str() {
                                if full_str.contains("contract") {
                                    // Must be used as base class
                                    if all_bases.contains(alias) || real_usages.contains(alias) {
                                        satisfied = true;
                                    }
                                }
                            }
                        }
                    } else {
                        // General layers
                        for (alias, fullname) in &aliases {
                            if let Some(full_str) = fullname.as_str() {
                                let detected = self.detect_module_layer(full_str);
                                if let Some(ref d) = detected {
                                    if d == req_layer && real_usages.contains(alias) {
                                        satisfied = true;
                                    }
                                }
                            }
                        }
                    }

                    if !satisfied {
                        missing_mandatory.push(req_layer.clone());
                    }
                }

                // Check mandatory suffix rule imports
                let stem_without_ext = filename.replace(".py", "");
                for rule_vo in &def.mandatory_imports {
                    let marker = &rule_vo.suffix.value;
                    if stem_without_ext.contains(marker) {
                        for req_layer in &rule_vo.imports.values {
                            let mut satisfied = false;
                            
                            if req_layer.starts_with("contract") {
                                for (alias, fullname) in &aliases {
                                    if let Some(full_str) = fullname.as_str() {
                                        if full_str.contains("contract") {
                                            if all_bases.contains(alias) || real_usages.contains(alias) {
                                                satisfied = true;
                                            }
                                        }
                                    }
                                }
                            } else {
                                for (alias, fullname) in &aliases {
                                    if let Some(full_str) = fullname.as_str() {
                                        let detected = self.detect_module_layer(full_str);
                                        if let Some(ref d) = detected {
                                            if d == req_layer && real_usages.contains(alias) {
                                                satisfied = true;
                                            }
                                        }
                                    }
                                }
                            }

                            if !satisfied {
                                missing_mandatory.push(req_layer.clone());
                            }
                        }
                    }
                }
            }
        }

        if !missing_mandatory.is_empty() {
            let msg = if !def.mandatory_import_violation_message.value.is_empty() {
                def.mandatory_import_violation_message.value.clone()
            } else {
                format!(
                    "Mandatory layer import is missing: required layers [{}].",
                    missing_mandatory.join(", ")
                )
            };
            violations.push(LintResult {
                file: FilePath::new(file.to_string()),
                line: LineNumber::new(0),
                column: ColumnNumber::new(0),
                code: ErrorCode::new("AES002"),
                message: LintMessage::new(msg),
                source: AdapterName::new("architecture"),
                severity: Severity::HIGH,
                enclosing_scope: ScopeRef {
                    name: "".to_string(),
                    kind: "".to_string(),
                    file: FilePath::new(""),
                    start_line: LineNumber::new(0),
                    end_line: LineNumber::new(0),
                },
                related_locations: LocationList::new(Vec::new()),
            });
        }

        // 2. Forbidden / allowed imports check (AES001)
        if !def.forbidden_import.values.is_empty() || !def.allowed_import.values.is_empty() {
            if let Ok(imports) = scanner.extract_imports(&filepath) {
                for imp in imports.values {
                    let mod_str = imp.module.value.clone();
                    let target_layer = self.detect_module_layer(&mod_str);
                    if let Some(ref tl) = target_layer {
                        // Forbidden matches
                        let is_forbidden = def.forbidden_import.values.contains(tl);
                        let is_allowed = def.allowed_import.values.is_empty() || def.allowed_import.values.contains(tl) || layer_name.as_ref() == Some(tl);

                        if is_forbidden || !is_allowed {
                            let msg = if !def.forbidden_import_violation_message.value.is_empty() {
                                def.forbidden_import_violation_message.value.clone()
                            } else {
                                "Forbidden layer import detected.".to_string()
                            };
                            violations.push(LintResult {
                                file: FilePath::new(file.to_string()),
                                line: LineNumber::new(imp.line.value),
                                column: ColumnNumber::new(0),
                                code: ErrorCode::new("AES001"),
                                message: LintMessage::new(msg),
                                source: AdapterName::new("architecture"),
                                severity: Severity::CRITICAL,
                                enclosing_scope: ScopeRef {
                                    name: "".to_string(),
                                    kind: "".to_string(),
                                    file: FilePath::new(""),
                                    start_line: LineNumber::new(0),
                                    end_line: LineNumber::new(0),
                                },
                                related_locations: LocationList::new(Vec::new()),
                            });
                        }
                    }
                }
            }
        }

        // 3. Legacy layer rules checks (AES001)
        if !self.config.governance_rules.values.is_empty() {
            if let Some(file_layer) = layer_name {
                if file_layer != "agent" && !file_layer.starts_with("agent(") {
                    if let Ok(imports) = scanner.extract_imports(&filepath) {
                        for imp in imports.values {
                            let mod_str = imp.module.value.clone();
                            let target_layer = self.detect_module_layer(&mod_str);
                            if let Some(ref tl) = target_layer {
                                for rule in &self.config.governance_rules.values {
                                    if file_layer == &rule.source_layer.value && tl == &rule.forbidden_target.value {
                                        let desc = if rule.description.value.is_empty() {
                                            "Forbidden layer import detected.".to_string()
                                        } else {
                                            rule.description.value.clone()
                                        };
                                        let base_msg = format!(
                                            "[AES Layer Violation] {}. File in '{}' imports from '{}' via '{}'.",
                                            desc, file_layer, tl, mod_str
                                        );
                                        violations.push(LintResult {
                                            file: FilePath::new(file.to_string()),
                                            line: LineNumber::new(imp.line.value),
                                            column: ColumnNumber::new(0),
                                            code: ErrorCode::new("AES001"),
                                            message: LintMessage::new(base_msg),
                                            source: AdapterName::new("architecture"),
                                            severity: Severity::CRITICAL,
                                            enclosing_scope: ScopeRef {
                                                name: "".to_string(),
                                                kind: "".to_string(),
                                                file: FilePath::new(""),
                                                start_line: LineNumber::new(0),
                                                end_line: LineNumber::new(0),
                                            },
                                            related_locations: LocationList::new(Vec::new()),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
