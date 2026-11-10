// PURPOSE: SuffixPolicyChecker — AES102 suffix/prefix policy enforcement capability
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_layer_vo::{LayerDefinition, LayerMapVO};
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_path_vo::FilePathList;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_violation_item_vo::ViolationItem;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_naming_rules::contract_naming_checker_protocol::ISuffixPolicyProtocol;
use shared_naming_rules::taxonomy_naming_rules_constant::{
    AES102_SUFFIX_RULE, RULE_CODE_SUFFIX_PREFIX, SPECIALIZED_LAYER_MARKER, SUFFIX_POLICY_STRICT,
};
use shared_naming_rules::utility_naming_checker::{
    basename_of, detect_layer, get_stem, get_suffix, parse_path, rule_exception_set,
    string_filename_violation,
};
use shared_naming_rules::utility_naming_output::{exemption_clause, naming_finding};

use std::collections::HashMap;

// ─── Block 1: Module Declaration ──────────────────────────────

/// Stateless AES102 suffix/prefix policy checker.
///
/// Validates that each file's suffix matches its layer's allowed set, enforces
/// forbidden-suffix rules, and catches cross-layer suffix mismatches. No
/// internal state.
pub struct SuffixPolicyChecker {}

impl Default for SuffixPolicyChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SuffixPolicyChecker {
    pub fn new() -> Self {
        Self {}
    }
}

// ─── Block 2: Module Contract ────────────────

impl ISuffixPolicyProtocol for SuffixPolicyChecker {
    /// FR-NamingRules-002 — AES102: check each file's layer suffix and role
    /// prefix against the per-layer policy.
    fn check_domain_suffixes(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut Vec<ViolationItem>,
    ) {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let suffix_to_layer = Self::build_suffix_to_layer_map(layer_map);
        let exceptions = rule_exception_set(config, RULE_CODE_SUFFIX_PREFIX);

        let violations: Vec<ViolationItem> = files
            .values
            .par_iter()
            .filter_map(|f| {
                let f_str = f.to_string();
                let filename = basename_of(&f_str);
                // Rule-level exceptions evaluated before layer detection (FRD FR-002).
                if exceptions.iter().any(|v| v == filename) {
                    return None;
                }
                let layer = detect_layer(&f_str, &layer_keys);
                let layer_name = layer.as_ref().map(|l| LayerNameVO::new(l.clone()));

                // No recognised layer prefix → no suffix policy applies → skip.
                // (AES000 removed: unknown-prefix signalling is out of scope.)
                layer.as_ref()?;

                let def = layer_name.as_ref().and_then(|l| layer_map.values.get(l));
                self.check_domain_suffixes_internal(
                    &f_str,
                    filename,
                    def,
                    &layer_name,
                    &suffix_to_layer,
                )
            })
            .collect();

        results.extend(violations);
    }
}

// ─── Block 3: Module Helpers ────────────

impl SuffixPolicyChecker {
    /// Build a mapping from suffix → base layer name for cross-layer validation.
    ///
    /// B-6a test seam: `pub` only so cross-crate unit tests can inspect the
    /// suffix→layer table; production traffic enters through
    /// [`ISuffixPolicyProtocol::check_domain_suffixes`].
    pub fn build_suffix_to_layer_map(layer_map: &LayerMapVO) -> HashMap<String, String> {
        let mut suffix_to_layer = HashMap::new();
        for (layer_name, def) in &layer_map.values {
            // Sub-layers (e.g. "capabilities(command)") share their base layer's
            // suffix set, so skipping them here avoids redundant cross-layer entries.
            if layer_name.value().contains(SPECIALIZED_LAYER_MARKER) {
                continue;
            }
            if def.naming.suffix_policy.value == SUFFIX_POLICY_STRICT {
                for suffix in &def.naming.allowed_suffix.values {
                    suffix_to_layer
                        .entry(suffix.clone())
                        .or_insert_with(|| layer_name.value().to_string());
                }
            }
        }
        suffix_to_layer
    }

    /// Check domain suffix rules per layer (AES102: suffix/prefix rules + cross-layer validation).
    ///
    /// B-6a test seam: `pub` only so cross-crate unit tests can drive the
    /// per-file rule; production traffic enters through
    /// [`ISuffixPolicyProtocol::check_domain_suffixes`].
    pub fn check_domain_suffixes_internal(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&LayerDefinition>,
        layer_name: &Option<LayerNameVO>,
        suffix_to_layer: &HashMap<String, String>,
    ) -> Option<ViolationItem> {
        let fp = parse_path(filename)?;
        if fp.is_barrel_file() || fp.is_entry_point() {
            return None;
        }

        let def = definition?;
        if def.exceptions.values.iter().any(|v| v == filename) {
            return None;
        }

        let stem = get_stem(filename)?;
        let suffix = get_suffix(stem);
        // The folder the file sits in, named the way a reader sees it: a
        // specialized layer reads `capabilities(command)`, but the folder on
        // disk is `capabilities`.
        let layer_text = layer_name
            .as_ref()
            .map(|l| l.value().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let layer_display = base_layer_of(&layer_text).to_string();
        let allowed: &[String] = &def.naming.allowed_suffix.values;

        // 1. Forbidden suffix check (always enforced regardless of policy)
        if let Some(suf) = &suffix
            && def.naming.forbidden_suffix.values.iter().any(|v| v == *suf)
        {
            let owner = suffix_to_layer.get(*suf).map(|owner| owner.as_str());
            return Some(Self::forbidden_suffix_violation(
                file,
                filename,
                stem,
                suf,
                layer_display.as_str(),
                allowed,
                owner,
            ));
        }

        // 2. Cross-layer suffix validation (FR-002: PrefixSuffixMismatch)
        if let Some(suf) = &suffix
            && let Some(suffix_belonging_layer) = suffix_to_layer.get(*suf)
        {
            if suffix_belonging_layer.as_str() != layer_display.as_str() {
                return Some(Self::cross_layer_violation(
                    file,
                    filename,
                    stem,
                    suf,
                    layer_display.as_str(),
                    suffix_belonging_layer.as_str(),
                ));
            }
        }

        // 3. Strict policy check (suffix not in this layer's allowed list)
        if def.naming.suffix_policy.value == SUFFIX_POLICY_STRICT {
            let valid = match &suffix {
                Some(s) => def.naming.allowed_suffix.values.iter().any(|v| v == s),
                None => false,
            };
            if !valid {
                return Some(Self::unknown_suffix_violation(
                    file,
                    filename,
                    stem,
                    suffix,
                    layer_display.as_str(),
                    allowed,
                ));
            }
        }

        None
    }

    /// The name the file should carry when it belongs to *owner*.
    ///
    /// The layer a file belongs to is read from its own name, so a file that
    /// plays another folder's role takes that folder's name: the leading
    /// folder word is replaced and the rest of the name is kept, lowercased
    /// and tidied into words the naming rules accept. `None` when the name
    /// carries nothing to keep.
    fn target_name(stem: &str, folder: &str, owner: &str, filename: &str) -> Option<String> {
        let rest = stem
            .strip_prefix(folder)
            .map(|rest| rest.strip_prefix('_').unwrap_or(rest))
            .unwrap_or(stem);
        let rest = Self::words_of(rest);
        if rest.is_empty() {
            return None;
        }
        let target = format!("{owner}_{}", rest.join("_"));
        match std::path::Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
        {
            Some(extension) => Some(format!("{target}.{extension}")),
            None => Some(target),
        }
    }

    /// The words a name holds, lowercased and tidied.
    ///
    /// A character a file name may not hold becomes a word break, and a run of
    /// breaks collapses, so the words are the ones the naming rules accept.
    fn words_of(name: &str) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        let mut current = String::new();
        for character in name.chars() {
            if character.is_ascii_alphanumeric() {
                current.push(character.to_ascii_lowercase());
            } else if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
        }
        if !current.is_empty() {
            words.push(current);
        }
        words
    }

    /// The finding for a suffix the folder forbids outright.
    ///
    /// The suffix names a role the folder does not play, so the name is wrong
    /// in this folder: the finding outranks a suffix that is merely unknown.
    fn forbidden_suffix_violation(
        file: &str,
        filename: &str,
        stem: &str,
        suffix: &str,
        layer_display: &str,
        allowed: &[String],
        owner: Option<&str>,
    ) -> ViolationItem {
        let problem = format!("Suffix '{suffix}' is not allowed in the '{layer_display}' folder.");
        // One action per finding. The suffix names a role another folder
        // plays, so the file takes that folder's name. A suffix no other
        // folder owns is dropped, or replaced by one this folder lists.
        let action = match owner {
            Some(owner) if owner != layer_display => {
                match Self::target_name(stem, layer_display, owner, filename) {
                    Some(target) => format!("Rename the file to '{target}'."),
                    None => Self::suffix_action(suffix, allowed),
                }
            }
            _ => Self::suffix_action(suffix, allowed),
        };
        let exemption = exemption_clause(RULE_CODE_SUFFIX_PREFIX, filename);
        let finding = naming_finding(
            &[problem.as_str()],
            &[
                action.as_str(),
                "Then update the imports that name it.",
                "Re-run the check to confirm the finding is gone.",
                exemption.as_str(),
            ],
            &[AES102_SUFFIX_RULE],
        );
        string_filename_violation(
            file,
            RULE_CODE_SUFFIX_PREFIX,
            finding.message,
            Severity::HIGH,
            "SUFFIX_PREFIX",
            finding.why,
            finding.fix,
        )
    }

    /// The action for a suffix the folder does not accept.
    ///
    /// A folder that lists the suffixes it accepts names them here, so the
    /// rename is a pick-one. A folder that lists none accepts no suffix at
    /// all, so the suffix has to go.
    fn suffix_action(suffix: &str, allowed: &[String]) -> String {
        if allowed.is_empty() {
            format!("Rename the file so its name does not end with the '{suffix}' suffix.")
        } else {
            format!(
                "Rename the file so its name ends with one of: {}.",
                allowed.join(", ")
            )
        }
    }

    /// The finding for a suffix that belongs to another folder.
    ///
    /// The suffix is legal, but it is legal in the folder that owns it: the
    /// file plays a role its own folder does not play, so the name has to
    /// carry the folder that owns the role.
    fn cross_layer_violation(
        file: &str,
        filename: &str,
        stem: &str,
        suffix: &str,
        layer_display: &str,
        owner: &str,
    ) -> ViolationItem {
        let problem = format!(
            "Suffix '{suffix}' belongs to the '{owner}' folder, but this file is in \
             '{layer_display}'."
        );
        let action = match Self::target_name(stem, layer_display, owner, filename) {
            Some(target) => format!("Rename the file to '{target}'."),
            None => format!(
                "Rename the file so its name starts with '{owner}_' and ends with '{suffix}'."
            ),
        };
        let exemption = exemption_clause(RULE_CODE_SUFFIX_PREFIX, filename);
        let finding = naming_finding(
            &[problem.as_str()],
            &[
                action.as_str(),
                "Then update the imports that name it.",
                "Re-run the check to confirm the finding is gone.",
                exemption.as_str(),
            ],
            &[AES102_SUFFIX_RULE],
        );
        string_filename_violation(
            file,
            RULE_CODE_SUFFIX_PREFIX,
            finding.message,
            Severity::HIGH,
            "SUFFIX_PREFIX",
            finding.why,
            finding.fix,
        )
    }

    /// The finding for a suffix the folder does not list.
    ///
    /// The file sits in the right folder, so this is a naming problem rather
    /// than a placement one, and it ranks below the two findings above.
    fn unknown_suffix_violation(
        file: &str,
        filename: &str,
        stem: &str,
        suffix: Option<&str>,
        layer_display: &str,
        allowed: &[String],
    ) -> ViolationItem {
        // A file with no suffix at all and a file with the wrong suffix are
        // one defect: the name does not end with a suffix the folder allows.
        let problem = match suffix {
            Some(suffix) => {
                format!("Suffix '{suffix}' is not in the list the '{layer_display}' folder allows.")
            }
            None => format!(
                "Stem '{stem}' does not end with a suffix the '{layer_display}' folder allows."
            ),
        };
        // The file sits in the right folder, so the name is what changes: it
        // must end with one of the suffixes the folder lists.
        let action = if allowed.is_empty() {
            "Rename the file so its name does not end with a suffix.".to_string()
        } else {
            format!(
                "Rename the file so its name ends with one of: {}.",
                allowed.join(", ")
            )
        };
        let exemption = exemption_clause(RULE_CODE_SUFFIX_PREFIX, filename);
        let finding = naming_finding(
            &[problem.as_str()],
            &[
                action.as_str(),
                "Then update the imports that name it.",
                "Re-run the check to confirm the finding is gone.",
                exemption.as_str(),
            ],
            &[AES102_SUFFIX_RULE],
        );
        string_filename_violation(
            file,
            RULE_CODE_SUFFIX_PREFIX,
            finding.message,
            Severity::MEDIUM,
            "SUFFIX_PREFIX",
            finding.why,
            finding.fix,
        )
    }
}

/// Extract the base layer name from a potentially specialized layer display string.
/// e.g. `"surface(command)"` → `"surface"`, `"utility"` → `"utility"`.
/// `split('(')` always yields at least one element, so `unwrap_or` is unreachable
/// but satisfies the linter's no-unwrap policy.
fn base_layer_of(layer_display: &str) -> &str {
    layer_display.split('(').next().unwrap_or(layer_display)
}
