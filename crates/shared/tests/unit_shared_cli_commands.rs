// Unit tests — shared/cli_commands taxonomy types.
use clap::Parser;
use shared_lint_arwaky::cli_commands::Format;
use shared_lint_arwaky::cli_commands::taxonomy_cli_vo::{Cli, Commands};
use shared_lint_arwaky::cli_commands::taxonomy_command_catalog_vo::{
    COMMAND_CATALOG, CommandCatalogVO,
};
use shared_lint_arwaky::cli_commands::taxonomy_protocol_vo::{
    TransportEndpoint, TransportProtocol, TransportUrlVO,
};
use shared_lint_arwaky::cli_commands::taxonomy_scan_report_vo::{
    DiagnosticSeverity, PipelineDiagnostic, PipelineError, ScanReport,
};
use shared_lint_arwaky::cli_commands::taxonomy_scan_request_vo::{
    ScanMode, ScanRequest, ScanTarget,
};
use shared_lint_arwaky::common::Score;
use shared_lint_arwaky::common::taxonomy_severity_vo::Severity;

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
        let cli = Cli::try_parse_from(full_args)
            .expect("valid clap args");
        assert_eq!(format!("{:?}", cli.command), expected);
    }
}

#[test]
fn cli_global_flags_propagate() {
    let cli = Cli::try_parse_from(["lint-arwaky", "--verbose", "--quiet", "doctor"])
        .expect("valid clap args");
    assert!(cli.verbose);
    assert!(cli.quiet);
}

// ── CommandCatalogVO ────────────────────────────────────────
#[test]
fn command_catalog_contains_core_commands() {
    let catalog = CommandCatalogVO::command_catalog();
    assert!(catalog.len() >= 13);
    for (name, _, _) in COMMAND_CATALOG {
        assert!(catalog.contains_key(&shared_lint_arwaky::common::ActionName::from(*name)));
    }
    let check = catalog.get(&shared_lint_arwaky::common::ActionName::from("check"));
    assert!(check.is_some());
    assert!(!check.unwrap().example.value.is_empty());
}

#[test]
fn command_metadata_display() {
    let catalog = CommandCatalogVO::command_catalog();
    let check = catalog
        .get(&shared_lint_arwaky::common::ActionName::from("check"))
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
    use shared_lint_arwaky::common::LintResult;
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
