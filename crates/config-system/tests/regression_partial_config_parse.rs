// Regression tests — QA #640 (BE-cycle CRITICAL defect guard).
//
// Defect: a config document containing one malformed field among otherwise
// valid fields could silently wipe the entire parsed config back to defaults,
// with no error and no warning. The corrected contract (either is acceptable):
//   (a) salvage the valid fields and report a warning, or
//   (b) reject the whole parse loudly (Err / non-empty warnings).
// What must NEVER happen is a silent empty/default config. These tests lock
// that contract in permanently (see TEST.md §2.0 Regression Test Convention).
mod common;

use config_system_lint_arwaky::capabilities_parser_provider::ConfigParserProvider;
use shared_common::FilePath;
use shared_config_system::IConfigMergeProtocol;
use std::fs;
use tempfile::TempDir;

fn make_parser() -> ConfigParserProvider {
    ConfigParserProvider::new(common::make_io())
}

// QA #640 (TOML surface): 3 valid fields + 1 malformed `score.value` field.
// Passed today via behavior (b): `parse_toml_config` returns a loud
// ConfigError instead of an empty/default config.
#[test]
fn regression_640_toml_one_malformed_field_never_silently_wipes_config() {
    let tmp = TempDir::new().unwrap();
    let toml_content = r#"[tool.lint-arwaky]
project_name = "my-project"

[tool.lint-arwaky.thresholds]
score = { value = "not-a-number" }
complexity = { value = 8 }
max_file_lines = { value = 300 }
"#;
    let path = tmp.path().join("Cargo.toml");
    fs::write(&path, toml_content).unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();

    match make_parser().parse_toml_config(&fp) {
        Err(err) => {
            // Behavior (b): fail the whole parse loudly — the error must name
            // the problem, not pretend the document was absent.
            assert!(
                err.message.value.contains("Failed"),
                "parse error must be descriptive: {}",
                err.message.value
            );
        }
        Ok(Some(config)) => {
            // Behavior (a, once the BE fix lands): invalid field dropped, but
            // every valid sibling field must be retained.
            assert_eq!(
                config.thresholds.complexity.value, 8,
                "valid sibling field `complexity` must survive one bad field"
            );
            assert_eq!(
                config.thresholds.max_file_lines.value, 300,
                "valid sibling field `max_file_lines` must survive one bad field"
            );
            assert_eq!(
                config.project_name.value, "my-project",
                "valid sibling field `project_name` must survive one bad field"
            );
        }
        Ok(None) => {
            panic!(
                "silent wipe detected: document with [tool.lint-arwaky] returned \
                 Ok(None) as if no config existed"
            );
        }
    }
}

// QA #640 (YAML surface): one malformed field among valid siblings must
// produce either salvaged values or a loud warning — never silence.
#[test]
fn regression_640_yaml_one_malformed_field_never_silently_wipes_config() {
    let yaml = "architecture:\n  enabled: \"not-a-bool\"\n  mandatory_class_definition: true\n  rules: []\n";
    let (config, warnings) = make_parser().parse_config_yaml_with_warnings(yaml);

    let salvaged_valid_fields = config.mandatory_class_definition.value;
    assert!(
        !warnings.is_empty() || salvaged_valid_fields,
        "one malformed field must not silently wipe the YAML config: expected \
         either a loud warning (got none) or salvaged valid fields"
    );
}

// Control: a fully valid YAML document parses with its values intact.
#[test]
fn regression_640_yaml_valid_config_control() {
    let yaml = "architecture:\n  enabled: true\n  mandatory_class_definition: true\n  rules: []\n";
    let (config, _) = make_parser().parse_config_yaml_with_warnings(yaml);
    assert!(config.enabled.value, "valid config must parse its values");
    assert!(config.mandatory_class_definition.value, "valid config must parse its values");
}
