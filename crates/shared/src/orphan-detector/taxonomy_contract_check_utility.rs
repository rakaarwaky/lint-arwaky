use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::orphan_detector::contract_orphan_protocol::IOrphanFilenameExtractorProtocol;
use crate::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use std::collections::HashMap;

use super::taxonomy_contract_detection_utility::{
    extract_contract_trait_name, has_py_call, has_py_impl, has_py_wire, has_rust_call,
    has_rust_impl, has_rust_wire, has_ts_call, has_ts_impl, has_ts_wire,
};
use super::taxonomy_contract_regex_utility::word_boundary_re;

const SUFFIX_PORT: &str = "port";
const SUFFIX_PROTOCOL: &str = "protocol";
const SUFFIX_AGGREGATE: &str = "aggregate";
const LAYER_INFRASTRUCTURE: &str = "infrastructure";
const LAYER_CAPABILITIES: &str = "capabilities";
const LAYER_AGENT: &str = "agent";

pub fn check_implemented(
    contents: &HashMap<String, String>,
    basenames: &HashMap<String, String>,
    trait_name: &str,
    target_prefix: &str,
) -> bool {
    let rust_impl_pattern = format!("impl {} for", trait_name);
    let re_trait = word_boundary_re(trait_name);

    for (path, content) in contents {
        let bn = match basenames.get(path) {
            Some(b) => b.as_str(),
            None => continue,
        };
        let is_target_layer = bn.starts_with(target_prefix);
        let is_container_impl = bn.starts_with("root_")
            && (bn.ends_with("_container.rs")
                || bn.ends_with("_container.py")
                || bn.ends_with("_container.ts")
                || bn.ends_with("_container.js"));
        if !is_target_layer && !is_container_impl {
            continue;
        }
        if has_rust_impl(content, &rust_impl_pattern, &re_trait)
            || has_py_impl(content, trait_name)
            || has_ts_impl(content, trait_name)
        {
            return true;
        }
    }
    false
}

pub fn check_called(
    contents: &HashMap<String, String>,
    basenames: &HashMap<String, String>,
    trait_name: &str,
) -> bool {
    let re_trait = word_boundary_re(trait_name);

    for (path, content) in contents {
        let bn = match basenames.get(path) {
            Some(b) => b.as_str(),
            None => continue,
        };
        let is_relevant = bn.starts_with("agent_")
            || bn.ends_with("_container.rs")
            || bn.ends_with("_container.py")
            || bn.ends_with("_container.ts")
            || bn.ends_with("_container.js")
            || bn.starts_with("capabilities_")
            || bn.starts_with("surface_");

        if !is_relevant {
            continue;
        }
        if has_rust_call(content, &re_trait)
            || has_py_call(content, &re_trait)
            || has_ts_call(content, &re_trait)
        {
            return true;
        }
    }
    false
}

pub fn check_wired(
    contents: &HashMap<String, String>,
    basenames: &HashMap<String, String>,
    trait_name: &str,
) -> bool {
    let re_trait = word_boundary_re(trait_name);

    for (path, content) in contents {
        let bn = match basenames.get(path) {
            Some(b) => b.as_str(),
            None => continue,
        };
        let is_relevant = bn.starts_with("agent_")
            || bn.ends_with("_container.rs")
            || bn.ends_with("_container.py")
            || bn.ends_with("_container.ts")
            || bn.ends_with("_container.js")
            || bn.starts_with("capabilities_")
            || bn.starts_with("surface_");

        if !is_relevant {
            continue;
        }
        if has_rust_wire(content, &re_trait)
            || has_py_wire(content, &re_trait)
            || has_ts_wire(content, &re_trait)
        {
            return true;
        }
    }
    false
}

pub fn not_orphan() -> OrphanIndicatorResult {
    OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
}

pub fn orphan_result(
    suffix: &str,
    trait_name: &str,
    target_prefix: &str,
    reason: &str,
) -> OrphanIndicatorResult {
    OrphanIndicatorResult::new(
        true,
        AesOrphanViolation::ContractOrphan {
            suffix: suffix.to_string(),
            trait_name: trait_name.to_string(),
            target_layer: target_prefix.to_string(),
            reason: Some(reason.to_string().into()),
        }
        .to_string(),
        Severity::LOW,
    )
}

pub fn is_contract_orphan(
    f: &FilePath,
    contents: &HashMap<String, String>,
    basenames: &HashMap<String, String>,
    extractor: &dyn IOrphanFilenameExtractorProtocol,
) -> OrphanIndicatorResult {
    let fp = f.value();
    let suffix = extractor.file_suffix(f).value;

    let content = match contents.get(fp) {
        Some(c) => c.as_str(),
        None => return not_orphan(),
    };

    let trait_name = match extract_contract_trait_name(content, fp) {
        Some(t) => t,
        None => return not_orphan(),
    };

    let target_prefix = match suffix.as_str() {
        SUFFIX_PORT => LAYER_INFRASTRUCTURE,
        SUFFIX_PROTOCOL => LAYER_CAPABILITIES,
        SUFFIX_AGGREGATE => LAYER_AGENT,
        _ => return not_orphan(),
    };

    if !check_implemented(contents, basenames, &trait_name, target_prefix) {
        return orphan_result(
            &suffix,
            &trait_name,
            target_prefix,
            &format!(
                "Contract {} '{}' not implemented by any {} file.",
                suffix, trait_name, target_prefix
            ),
        );
    }

    if (suffix == SUFFIX_PORT || suffix == SUFFIX_PROTOCOL)
        && !check_called(contents, basenames, &trait_name)
    {
        return orphan_result(
            &suffix,
            &trait_name,
            target_prefix,
            &format!(
                "Contract {} '{}' not called by any orchestrator, container, capabilities, or surface file.",
                suffix, trait_name
            ),
        );
    }

    if (suffix == SUFFIX_PORT || suffix == SUFFIX_PROTOCOL)
        && !check_wired(contents, basenames, &trait_name)
    {
        return orphan_result(
            &suffix,
            &trait_name,
            target_prefix,
            &format!(
                "Contract {} '{}' not wired in any DI container (no Arc::new, Box::new, or constructor injection).",
                suffix, trait_name
            ),
        );
    }

    if suffix == SUFFIX_AGGREGATE && !check_called(contents, basenames, &trait_name) {
        return orphan_result(
            &suffix,
            &trait_name,
            target_prefix,
            &format!(
                "Contract aggregate '{}' not called by any surface or container file.",
                trait_name
            ),
        );
    }

    if suffix == SUFFIX_AGGREGATE && !check_wired(contents, basenames, &trait_name) {
        return orphan_result(
            &suffix,
            &trait_name,
            target_prefix,
            &format!(
                "Contract aggregate '{}' not wired in any DI container (no Arc::new, Box::new, or constructor injection).",
                trait_name
            ),
        );
    }

    not_orphan()
}
