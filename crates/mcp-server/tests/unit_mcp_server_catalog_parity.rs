// BE-1-02 — the advertised command list and the dispatch table must agree.
//
// `COMMAND_CATALOG` advertised `config` and `git`, and the MCP dispatch had an
// arm for neither: `config` answered "Unknown action" (exit 2) while `git` was
// absent entirely, so a caller following the catalog reached a runtime error.
//
// The dispatch arms live in a match on `&str`, so this test pins the two halves
// that are visible from outside: every catalogued name must be accepted, and the
// verbs that are served but deliberately not catalogued must stay served.
use mcp_server_lint_arwaky::taxonomy_mcp_server_vo::ExecuteCommandArgs;
use shared_cli_commands::taxonomy_cli_commands_vo::COMMAND_CATALOG;

/// Names the router serves. Kept here rather than reaching into the private
/// match so the list is explicit: adding a verb means updating this list and
/// the catalog together, which is the whole point.
const DISPATCHED: &[&str] = &[
    "check",
    "scan",
    "fix",
    "ci",
    "doctor",
    "config",
    "config-show",
    "git",
    "security",
    "dependencies",
    "quality",
    "import",
    "naming",
    "orphan",
    "role",
    "structure",
    "external",
    "docs",
    "docs_scaffold",
    "taxonomy",
    "contract",
    "capabilities",
    "utility",
    "agents",
    "surface",
    "version",
    "watch",
    "report",
    "adapters",
    "install-hook",
    "uninstall-hook",
    "init",
    "install",
    "mcp-config",
];

#[test]
fn every_catalogued_command_is_dispatchable() {
    for (name, _, _) in COMMAND_CATALOG {
        assert!(
            DISPATCHED.contains(name),
            "`{name}` is advertised in COMMAND_CATALOG but has no dispatch arm, \
             so a caller following the catalog reaches exit 2"
        );
    }
}

#[test]
fn the_catalog_deliberately_omits_scan_groups() {
    // Recorded decision: the per-group and per-layer verbs are *scan groups*,
    // not commands, so they are reachable as `execute_command` actions but not
    // advertised in the catalog. This test pins both halves of that decision
    // so neither side can drift silently: dispatchable, and not advertised.
    for name in [
        "quality",
        "role",
        "import",
        "naming",
        "orphan",
        "external",
        "docs",
        "structure",
        "taxonomy",
        "contract",
        "capabilities",
        "utility",
        "agents",
        "surface",
    ] {
        assert!(
            DISPATCHED.contains(&name),
            "`{name}` must stay dispatchable even though it is not catalogued"
        );
        assert!(
            !COMMAND_CATALOG.iter().any(|(listed, _, _)| *listed == name),
            "`{name}` is a scan group; advertising it in COMMAND_CATALOG would \
             contradict the documented catalog contract"
        );
    }
}

#[test]
fn the_catalog_has_no_duplicate_names() {
    let mut seen = std::collections::HashSet::new();
    for (name, _, _) in COMMAND_CATALOG {
        assert!(seen.insert(*name), "`{name}` is listed twice");
    }
}

#[test]
fn the_catalog_documents_every_entry() {
    // `list_commands` renders description and example straight into the tool
    // result; an empty one reaches the model as an empty string.
    for (name, description, example) in COMMAND_CATALOG {
        assert!(
            !description.trim().is_empty(),
            "`{name}` has no description"
        );
        assert!(!example.trim().is_empty(), "`{name}` has no usage example");
    }
}

#[test]
fn an_unknown_action_is_rejected_at_the_schema_boundary() {
    // Typed arguments mean a malformed call fails before dispatch, with a
    // message naming the key rather than "Unknown action".
    let result: Result<ExecuteCommandArgs, _> = serde_json::from_value(serde_json::json!({
        "action": "ci",
        "args": { "threshold": 80, "treshold": 90 }
    }));
    let error = result.expect_err("a misspelled argument must be rejected");
    assert!(error.to_string().contains("treshold"), "{error}");
}
