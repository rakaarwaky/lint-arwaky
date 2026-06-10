// PURPOSE: MandatoryInheritanceChecker — IMandatoryInheritanceProtocol for AES014: enforce contract implementation
use crate::code_analysis::contract_mandatory_inheritance_protocol::IMandatoryInheritanceProtocol;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use std::path::Path;

pub struct MandatoryInheritanceChecker {}

impl Default for MandatoryInheritanceChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl MandatoryInheritanceChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl IMandatoryInheritanceProtocol for MandatoryInheritanceChecker {
    fn check_mandatory_inheritance(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let contract_suffix = if layer == "infrastructure" || layer.starts_with("infrastructure(") {
            "_port"
        } else if layer == "capabilities" || layer.starts_with("capabilities(") {
            "_protocol"
        } else if layer == "agent" || layer.starts_with("agent(") {
            "_aggregate"
        } else {
            return;
        };

        let filename = Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let stem = filename.rsplit('.').next_back().unwrap_or(filename);
        let own_suffix = stem.rsplit('_').next().unwrap_or("");
        let implementer_suffixes = [
            "adapter",
            "provider",
            "scanner",
            "client",
            "gateway",
            "repository",
            "connector",
            "cache",
            "loader",
            "writer",
            "reader",
            "driver",
            "analyzer",
            "checker",
            "processor",
            "evaluator",
            "resolver",
            "validator",
            "formatter",
            "executor",
            "transformer",
            "builder",
            "compiler",
            "aggregator",
            "classifier",
            "extractor",
            "reporter",
            "mapper",
            "filter",
            "collector",
            "comparator",
            "scorer",
            "inspector",
            "reviewer",
            "assessor",
            "actions",
        ];
        let is_implementer = implementer_suffixes.contains(&own_suffix);
        if !is_implementer {
            return;
        }

        let mut imported: Vec<String> = Vec::new();
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("use ") && t.contains(contract_suffix) {
                if let Some(name) = t.split("::").last() {
                    let c = name.trim_end_matches(';').trim();
                    if c.starts_with('I') || c.ends_with("Protocol") || c.ends_with("Port") {
                        imported.push(c.to_string());
                    }
                }
            }
        }
        let has_impl = imported
            .iter()
            .any(|t| content.contains(&format!("impl {} for ", t)));
        if !has_impl {
            let all_are_deps: bool = imported.iter().all(|t| {
                content.contains(&format!("Arc<dyn {}>", t))
                    || content.contains(&format!("Box<dyn {}>", t))
                    || content.contains(&format!("&dyn {}", t))
                    || content.contains(&format!("&dyn mut {}", t))
            });
            if !all_are_deps {
                for t in &imported {
                    if !content.contains(&format!("Arc<dyn {}>", t))
                        && !content.contains(&format!("Box<dyn {}>", t))
                        && !content.contains(&format!("&dyn {}", t))
                        && !content.contains(&format!("&dyn mut {}", t))
                    {
                        violations.push(LintResult::new_arch(
                            file,
                            0,
                            "AES014",
                            Severity::HIGH,
                            &aes014_mandatory_inheritance(t),
                        ));
                    }
                }
            }
        }
    }
}

fn aes014_mandatory_inheritance(contracts: &str) -> String {
    format!("AES014 MANDATORY_INHERITANCE: File imports contracts ({}) but no class inherits from them.\nWHY? Layers that import contracts must provide an implementation.\nFIX: Add impl TraitName for YourStruct.", contracts)
}
