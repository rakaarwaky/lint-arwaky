// PURPOSE: DependencyCycleAnalyzer — ICycleAnalysisProtocol for AES205: circular dependency detection

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_cycle_protocol::ICycleAnalysisProtocol;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::DependencyEdge;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_lint_vo::ScopeRef;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_suggestion_vo::DescriptionVO;
use std::collections::HashMap;
use std::sync::Arc;

fn aes205_circular_import(source: &str, target: &str) -> String {
    format!(
        "AES205 CIRCULAR_IMPORT: Circular dependency detected: '{}' -> '{}'.",
        source, target
    )
}

/// Detects circular imports and dependency cycles (Capability).
pub struct DependencyCycleAnalyzer {
    _config: ArchitectureConfig,
    parser: Arc<dyn IImportParserPort>,
}

impl DependencyCycleAnalyzer {
    pub fn new(config: ArchitectureConfig, parser: Arc<dyn IImportParserPort>) -> Self {
        Self {
            _config: config,
            parser,
        }
    }

    fn make_result(file: &str, msg: &str) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(1),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw("AES205"),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: Severity::CRITICAL,
            enclosing_scope: Some(ScopeRef {
                name: DescriptionVO::new(String::new()),
                kind: DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    pub fn scan(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        let config = analyzer.config();
        if !config.enabled.value {
            return vec![];
        }

        let aes205_rule = config.rules.iter().find(|r| r.name.value == "AES205");

        let mut edges = Vec::new();
        let mut file_by_layer: HashMap<String, String> = HashMap::new();

        for file in files {
            let file_fp = FilePath::new(file.clone()).unwrap_or_default();
            let basename = file_fp.basename();
            if let Some(rule) = aes205_rule {
                if rule.exceptions.values.contains(&basename.to_string()) {
                    continue;
                }
            }

            let Ok(content) = self.parser.read_file_to_string(&file_fp) else {
                continue;
            };
            let file_fp = FilePath::new(file.clone()).unwrap_or_default();
            let file_layer = match analyzer.detect_layer(
                &file_fp,
                &FilePath::new(root_dir.to_string()).unwrap_or_default(),
            ) {
                Some(l) => l.value().to_string(),
                None => continue,
            };

            file_by_layer
                .entry(file_layer.clone())
                .or_insert_with(|| file.clone());

            let modules = self.parser.extract_import_modules(&content);
            for module in modules {
                let module_fp = FilePath::new(module.clone()).unwrap_or_default();
                if let Some(target_layer) = analyzer.detect_module_layer(&module_fp) {
                    let target_layer_str = target_layer.value().to_string();
                    if target_layer_str != file_layer {
                        edges.push(DependencyEdge::new(file_layer.clone(), target_layer_str));
                    }
                }
            }
        }

        let cycle_edge_results = self.parser.detect_cycle_edges(&edges);

        cycle_edge_results
            .into_iter()
            .map(|sn| {
                let edge_key = sn.value;
                let parts: Vec<&str> = edge_key.split("->").collect();
                let source = parts[0];
                let target = parts[1];
                let file = file_by_layer
                    .get(source)
                    .cloned()
                    .unwrap_or_else(|| source.to_string());
                let msg = aes205_circular_import(source, target);
                Self::make_result(&file, &msg)
            })
            .collect()
    }
}

#[async_trait]
impl ICycleAnalysisProtocol for DependencyCycleAnalyzer {
    async fn check_cycles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let file_strs: Vec<String> = files.values.iter().map(|f| f.to_string()).collect();
        let cycle_violations = self.scan(analyzer, &file_strs, &root_dir.to_string());
        results.values.extend(cycle_violations);
    }
}
