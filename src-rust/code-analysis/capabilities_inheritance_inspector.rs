//! Inline inheritance checks — dead inheritance and mandatory inheritance.

use std::path::Path;

use crate::code_analysis::capabilities_checker_helpers::mk_result;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violationrs_constant::{
    aes014_mandatory_inheritance, aes024_dead_inheritance,
};

/// Check for dead inheritance (empty struct/impl blocks).
pub fn check_dead_inheritance(file: &str, content: &str, violations: &mut Vec<LintResult>) {
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0usize;
    while i < lines.len() {
        let t = lines[i].trim();
        if t.starts_with("struct ") && t.ends_with(';') {
            violations.push(mk_result(
                file,
                i + 1,
                "AES024",
                Severity::MEDIUM,
                &aes024_dead_inheritance("unit struct"),
            ));
            i += 1;
            continue;
        }
        if t.starts_with("impl ") {
            let mut impl_str = t.to_string();
            let mut j = i;
            while !impl_str.contains(" for ") && j + 1 < lines.len() {
                j += 1;
                impl_str.push_str(lines[j].trim());
            }
            if impl_str.contains(" for ") {
                if impl_str.trim().ends_with("{}") {
                    violations.push(mk_result(
                        file,
                        i + 1,
                        "AES024",
                        Severity::MEDIUM,
                        &aes024_dead_inheritance("impl block"),
                    ));
                } else {
                    let mut k = j;
                    while k < lines.len() && !impl_str.contains('{') {
                        k += 1;
                        if k < lines.len() {
                            impl_str.push(' ');
                            impl_str.push_str(lines[k].trim());
                        }
                    }
                    if impl_str.trim().ends_with("{}") {
                        violations.push(mk_result(
                            file,
                            i + 1,
                            "AES024",
                            Severity::MEDIUM,
                            &aes024_dead_inheritance("impl block (multi-line)"),
                        ));
                    }
                }
            }
        }
        i += 1;
    }
}

/// Check that implementer files (adapter, provider, checker, etc.) implement
/// the contracts they import.
pub fn check_mandatory_inheritance(
    file: &str,
    content: &str,
    violations: &mut Vec<LintResult>,
) {
    let filename = Path::new(file)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let contract_suffix = if filename.starts_with("infrastructure_") {
        "_port"
    } else if filename.starts_with("capabilities_") {
        "_protocol"
    } else if filename.starts_with("agent_") {
        "_aggregate"
    } else {
        return;
    };

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
                    violations.push(mk_result(
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
