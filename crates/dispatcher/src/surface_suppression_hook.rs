// PURPOSE: Config resolution is scoped to the discovered workspace root,
// never the raw CLI target, so a member-scoped scan can't bind to a foreign
// parent project's config.
use shared_config_system::{
    IConfigFileReaderAggregate, taxonomy_config_system_vo::ArchitectureConfig,
};

use crate::surface_check_action::FilesystemSeam;

/// Returns the set of rule codes that are suppressed (either explicitly
/// disabled via `architecture.rules.<CODE>.enabled: false` or listed in the
/// top-level `ignored_rules` list) for doc/structure audit findings.
///
/// Resolves the discovered workspace root the structure/doc audit itself
/// resolves, then reads the nearest `lint_arwaky.config.yaml` at or above it
/// through the filesystem seam; an empty or missing config yields an empty
/// set (report everything).
///
/// `reader` is the `IConfigFileReaderAggregate` that performs the
/// `ignored_rules` walk (AES201: surface layer only imports
/// `contract(aggregate)` traits); the raw-content read for disabled rules is
/// local to this hook.
pub fn load_suppressed_rule_codes(
    root: &str,
    fs: &FilesystemSeam,
    reader: &dyn IConfigFileReaderAggregate,
) -> Vec<String> {
    let root_path = std::path::Path::new(root);
    let fs_ref = fs.aggregate.as_ref();
    let ws_root = fs_ref
        .execute(shared_filesystem::FilesystemRequest::find_workspace_root(
            root_path,
        ))
        .into_root()
        .unwrap_or_else(|| root_path.to_path_buf());
    let content = crate::surface_external_action::find_config_content(&ws_root, fs_ref);
    let mut codes = collect_suppressed_codes_from_content(content.as_deref());
    // `ignored_rules` is the top-level list; read it from the same config file
    // the rules came from so both keys stay in lockstep.
    let ignored = crate::surface_external_action::load_ignored_rules(&ws_root, fs_ref, reader);
    codes.extend(ignored);
    codes
}

/// Parse a config's disabled `architecture.rules.<CODE>` entries.
fn collect_suppressed_codes_from_content(content: Option<&str>) -> Vec<String> {
    let Some(content) = content else {
        return Vec::new();
    };
    let (config, _) =
        shared_config_system::utility_config_parser::parse_config_yaml_with_warnings(content);
    collect_suppressed_codes(&config)
}

/// Collects the rule codes that `architecture.rules.<CODE>.enabled: false`
/// disables in *config*.
pub fn collect_suppressed_codes(config: &ArchitectureConfig) -> Vec<String> {
    config
        .rules
        .iter()
        .filter(|rule| !rule.enabled.value())
        .map(|rule| rule.rule_type.code().to_string())
        .collect()
}

/// Whether a rule code is in the suppressed set — exact, case-insensitive
/// equality. A family prefix or typo (`AES`, `AES6`) never suppresses a
/// code it does not name.
pub fn is_rule_suppressed(codes: &[String], code: &str) -> bool {
    codes.iter().any(|c| c.eq_ignore_ascii_case(code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_suppressed_list_matches_nothing() {
        assert!(!is_rule_suppressed(&[], "AES601"));
    }

    #[test]
    fn exact_code_match_is_case_insensitive() {
        assert!(is_rule_suppressed(&["AES601".to_string()], "AES601"));
        assert!(is_rule_suppressed(&["aes601".to_string()], "AES601"));
    }

    #[test]
    fn family_prefix_does_not_suppress_another_code() {
        assert!(!is_rule_suppressed(&["AES".to_string()], "AES601"));
        assert!(!is_rule_suppressed(&["AES6".to_string()], "AES601"));
        assert!(is_rule_suppressed(&["AES601".to_string()], "AES601"));
    }

    #[test]
    fn collect_suppressed_codes_lists_disabled_rule_codes() {
        let config = shared_config_system::utility_config_parser::parse_config_yaml(
            "architecture:\n  rules:\n    AES702:\n      enabled: false\n",
        );
        let codes = collect_suppressed_codes(&config);
        assert!(codes.iter().any(|c| c == "AES702"));
        assert_eq!(codes.len(), 1);
    }

    #[test]
    fn load_suppressed_rule_codes_reads_both_keys_from_a_config_fixture() {
        let fixture = tempfile::tempdir().expect("tempdir");
        // A Cargo workspace marker (Cargo.toml + crates/ dir) lets
        // find_workspace_root detect the fixture as a workspace root,
        // so the config-file reader's workspace-boundary gate does not
        // short-circuit the walk.
        std::fs::create_dir_all(fixture.path().join("crates")).unwrap();
        std::fs::write(
            fixture.path().join("Cargo.toml"),
            "[workspace]\nmembers = [\"crates/*\"]\n",
        )
        .unwrap();
        std::fs::write(
            fixture
                .path()
                .join("lint_arwaky.config.yaml"),
            "architecture:\n  rules:\n    AES601:\n      enabled: false\nignored_rules:\n  - AES702\n",
        )
        .expect("write fixture config");

        let c = filesystem::root_filesystem_container::FilesystemContainer::new();
        let orchestrator = c.orchestrator();
        let fs_seam = crate::surface_check_action::FilesystemSeam {
            workspace: orchestrator.clone(),
            parser: orchestrator.clone(),
            aggregate: orchestrator,
        };
        let config_file_reader = config_system::root_config_system_container::ConfigContainer::new(
            fs_seam.aggregate.clone(),
            c.io(),
        );

        let codes = load_suppressed_rule_codes(
            fixture.path().to_string_lossy().as_ref(),
            &fs_seam,
            config_file_reader.file_reader().as_ref(),
        );
        assert!(
            codes.iter().any(|c| c.eq_ignore_ascii_case("AES601")),
            "AES601 (disabled via architecture.rules) must be suppressed: {codes:?}"
        );
        assert!(
            codes.iter().any(|c| c.eq_ignore_ascii_case("AES702")),
            "AES702 (via top-level ignored_rules) must be suppressed: {codes:?}"
        );
    }
}

pub struct _Aes303Placeholder {
    _placeholder: (),
}
