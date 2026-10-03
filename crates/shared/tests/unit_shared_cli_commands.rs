// Unit tests — shared/cli_commands taxonomy types.
use clap::Parser;
use shared_cli_commands::Format;
use shared_cli_commands::taxonomy_cli_commands_request::{ScanMode, ScanRequest, ScanTarget};
use shared_cli_commands::taxonomy_cli_commands_vo::{COMMAND_CATALOG, command_catalog};
use shared_cli_commands::taxonomy_cli_commands_vo::{Cli, Commands};
use shared_cli_commands::taxonomy_cli_commands_vo::{
    DiagnosticSeverity, PipelineDiagnostic, PipelineError, ScanReport,
};
use shared_cli_commands::taxonomy_cli_commands_vo::{
    TransportEndpoint, TransportProtocol, TransportUrlVO,
};
use shared_common::Score;
use shared_common::taxonomy_severity_vo::Severity;

// ── Cli / Commands (clap) ───────────────────────────────────
#[test]
fn cli_parses_scan_command() {
    let cli = Cli::try_parse_from(["lint-arwaky", "scan", "src/", "--format", "json"])
        .expect("valid clap args");
    assert!(!cli.verbose);
    match cli.command {
        Commands::Scan {
            path,
            format,
            member,
        } => {
            assert_eq!(path.as_deref(), Some("src/"));
            assert_eq!(format, Format::Json);
            assert!(member.is_none());
        }
        other => panic!("expected Scan, got {other:?}"),
    }
}

#[test]
fn cli_parses_scan_alias_check() {
    let cli = Cli::try_parse_from(["lint-arwaky", "check"]).expect("valid clap args");
    assert!(matches!(cli.command, Commands::Scan { .. }));
}

#[test]
fn cli_parses_fix_with_dry_run() {
    let cli = Cli::try_parse_from(["lint-arwaky", "fix", "--dry-run"]).expect("valid clap args");
    match cli.command {
        Commands::Fix { path, dry_run } => {
            assert!(path.is_none());
            assert!(dry_run);
        }
        other => panic!("expected Fix, got {other:?}"),
    }
}

#[test]
fn cli_parses_ci_with_threshold() {
    let cli = Cli::try_parse_from(["lint-arwaky", "ci", "--threshold", "90"]).expect("valid args");
    match cli.command {
        Commands::Ci { threshold, .. } => assert_eq!(threshold, 90),
        other => panic!("expected Ci, got {other:?}"),
    }
}

#[test]
fn cli_parses_flagless_commands() {
    for (args, expected) in [
        (vec!["doctor"], "Doctor"),
        (vec!["version"], "Version"),
        (vec!["init"], "Init"),
        (vec!["install-hook"], "InstallHook"),
        (vec!["uninstall-hook"], "UninstallHook"),
        (vec!["config-show"], "ConfigShow"),
    ] {
        let mut full_args = vec!["lint-arwaky"];
        full_args.extend_from_slice(&args);
        let cli = Cli::try_parse_from(full_args).expect("valid clap args");
        assert_eq!(format!("{:?}", cli.command), expected);
    }
}

#[test]
fn cli_parses_each_layer_scoped_command() {
    // One subcommand per AES layer except `root`: a layer command runs every
    // rule group and reports only that layer's files, so each must parse under
    // its own name and default to the current directory in text format.
    for (name, variant) in [
        ("taxonomy", "ScanTaxonomy"),
        ("contract", "ScanContract"),
        ("capabilities", "ScanCapabilities"),
        ("utility", "ScanUtility"),
        ("agents", "ScanAgents"),
        ("surface", "ScanSurface"),
    ] {
        let cli = Cli::try_parse_from(["lint-arwaky", name])
            .unwrap_or_else(|e| panic!("{name} must parse: {e}"));
        let debug = format!("{:?}", cli.command);
        assert!(
            debug.starts_with(variant),
            "{name} must map to {variant}, got {debug}"
        );
    }
}

#[test]
fn cli_parses_layer_scoped_command_with_path_and_format() {
    let cli = Cli::try_parse_from(["lint-arwaky", "capabilities", "src/", "--format", "json"])
        .expect("valid clap args");
    match cli.command {
        Commands::ScanCapabilities { path, format } => {
            assert_eq!(path.as_deref(), Some("src/"));
            assert_eq!(format, Format::Json);
        }
        other => panic!("expected ScanCapabilities, got {other:?}"),
    }
}

#[test]
fn cli_global_flags_propagate() {
    let cli = Cli::try_parse_from(["lint-arwaky", "--verbose", "--quiet", "doctor"])
        .expect("valid clap args");
    assert!(cli.verbose);
    assert!(cli.quiet);
}

// ── command_catalog ──────────────────────────────────────────
#[test]
fn command_catalog_contains_core_commands() {
    let catalog = command_catalog();
    assert!(catalog.len() >= 22);
    for (name, _, _) in COMMAND_CATALOG {
        assert!(catalog.contains_key(&shared_cli_commands::ActionName::from(*name)));
    }
    let check = catalog.get(&shared_cli_commands::ActionName::from("check"));
    assert!(check.is_some());
    assert!(!check.unwrap().example.value.is_empty());
}

#[test]
fn command_catalog_lists_every_layer_scoped_command() {
    // MCP `list_commands` advertises the catalog, so a layer command missing
    // from it would exist in the CLI but be undiscoverable over MCP.
    let catalog = command_catalog();
    for name in [
        "taxonomy",
        "contract",
        "capabilities",
        "utility",
        "agents",
        "surface",
    ] {
        let entry = catalog
            .get(&shared_cli_commands::ActionName::from(name))
            .unwrap_or_else(|| panic!("catalog must advertise '{name}'"));
        assert!(!entry.description.value.is_empty(), "{name}");
        assert!(!entry.example.value.is_empty(), "{name}");
    }
}

#[test]
fn command_metadata_display() {
    let catalog = command_catalog();
    let check = catalog
        .get(&shared_cli_commands::ActionName::from("check"))
        .expect("check exists");
    let rendered = check.to_string();
    assert!(rendered.contains('('));
}

// ── Transport endpoints ─────────────────────────────────────
#[test]
fn transport_endpoint_from_url() {
    assert_eq!(
        TransportEndpoint::from_url("https://localhost:8080").protocol,
        TransportProtocol::HTTP
    );
    assert_eq!(
        TransportEndpoint::from_url("/tmp/sock").protocol,
        TransportProtocol::UnixSocket
    );
    assert_eq!(
        TransportEndpoint::from_url("stdio").protocol,
        TransportProtocol::STDAggregate
    );
}

#[test]
fn transport_protocol_metadata() {
    assert!(TransportProtocol::HTTP.needs_desktop_commander());
    assert!(!TransportProtocol::STDAggregate.needs_desktop_commander());
    assert_eq!(TransportProtocol::UnixSocket.to_string(), "UnixSocket");
}

#[test]
fn transport_endpoint_display_name() {
    assert_eq!(
        TransportEndpoint::new(TransportProtocol::STDAggregate, "stdio".to_string()).display_name(),
        "Stdio(direct)"
    );
    assert_eq!(
        TransportEndpoint::new(TransportProtocol::HTTP, "h".to_string()).display_name(),
        "HTTP(h)"
    );
}

#[test]
fn transport_url_vo_wraps_string() {
    let url = TransportUrlVO::new("http://localhost");
    assert_eq!(url.value(), "http://localhost");
}

// ── ScanRequest / ScanMode / ScanTarget ─────────────────────
#[test]
fn scan_request_default_format() {
    let request = ScanRequest::new(ScanTarget::new(".".to_string()), ScanMode::Check);
    assert_eq!(request.format, Format::Text);
    assert!(request.filter.is_none());
}

#[test]
fn scan_target_default_is_dot() {
    assert_eq!(ScanTarget::default().value, ".");
}

#[test]
fn scan_mode_default_is_check() {
    assert!(matches!(ScanMode::default(), ScanMode::Check));
}

// ── ScanReport / diagnostics ────────────────────────────────
#[test]
fn scan_report_violation_count_ignores_info() {
    use shared_common::LintResult;
    let info = LintResult::new_arch("a.rs", 1, "AES101", Severity::INFO, "info");
    let high = LintResult::new_arch("b.rs", 1, "AES101", Severity::HIGH, "bad");
    let report = ScanReport::new(vec![info, high], Vec::new());
    assert_eq!(report.violation_count(), 1);
}

#[test]
fn scan_report_with_score() {
    let report = ScanReport::new(Vec::new(), Vec::new()).with_score(Score::new(88.0));
    assert_eq!(report.score.expect("score attached").value(), 88.0);
}

#[test]
fn pipeline_diagnostic_new() {
    let diag = PipelineDiagnostic::new(
        "config".to_string(),
        "warn".to_string(),
        DiagnosticSeverity::Warning,
    );
    assert_eq!(diag.source, "config");
}

// ── PARSE_WARN dedup invariant (issue #542) ───────────────

/// A parse failure recorded as both a `PARSE_`-prefixed `LintResult` in
/// `results` and a parser `PipelineDiagnostic` in `diagnostics` must
/// count once, not twice.
#[test]
fn scan_report_suppresses_parser_diagnostic_when_parse_result_exists() {
    use shared_common::LintResult;
    let parse_result = LintResult::new_arch(
        "src/bad.rs",
        1,
        "PARSE_WARN",
        Severity::MEDIUM,
        "File skipped: parse failure — syntax error",
    );
    let double_recorded = vec![
        LintResult::new_arch(
            "src/surface.rs",
            1,
            "AES201",
            Severity::CRITICAL,
            "surface -> capabilities import forbidden",
        ),
        parse_result,
    ];
    let parse_diagnostic = PipelineDiagnostic::new(
        "parser".to_string(),
        "File skipped: parse failure — syntax error".to_string(),
        DiagnosticSeverity::Warning,
    );
    let other_diagnostic = PipelineDiagnostic::new(
        "config".to_string(),
        "no config found".to_string(),
        DiagnosticSeverity::Info,
    );

    let report = ScanReport::new(double_recorded, vec![parse_diagnostic, other_diagnostic]);

    // The parser diagnostic is suppressed; the unrelated one survives.
    assert_eq!(report.diagnostics.len(), 1);
    assert!(report.diagnostics.iter().all(|d| d.source != "parser"));
    // Both results (incl. the PARSE_ one) still present — the count
    // reflects each finding once.
    assert_eq!(report.results.len(), 2);
    assert_eq!(report.violation_count(), 2);

    // No PARSE_ result: the parser diagnostic is kept (legacy path intact).
    let no_parse = vec![LintResult::new_arch(
        "src/surface.rs",
        1,
        "AES201",
        Severity::CRITICAL,
        "bad import",
    )];
    let report2 = ScanReport::new(
        no_parse,
        vec![PipelineDiagnostic::new(
            "parser".to_string(),
            "File skipped: parse failure — syntax error".to_string(),
            DiagnosticSeverity::Warning,
        )],
    );
    assert_eq!(report2.diagnostics.len(), 1);
}

#[test]
fn pipeline_error_display() {
    assert!(
        PipelineError::PathNotFound("/x".to_string())
            .to_string()
            .contains("path not found")
    );
    assert!(
        PipelineError::Io("e".to_string())
            .to_string()
            .contains("io error")
    );
}
