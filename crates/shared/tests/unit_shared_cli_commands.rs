// Unit tests — shared/cli_commands taxonomy types.
use clap::Parser;
use shared_cli_commands::Format;
use shared_cli_commands::ScanGroup;
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
            group,
        } => {
            assert_eq!(path.as_deref(), Some("src/"));
            assert_eq!(format, Format::Json);
            assert!(member.is_none());
            assert!(group.is_none());
        }
        other => panic!("expected Scan, got {other:?}"),
    }
    assert!(
        cli.rules.is_none(),
        "rules must default to None when omitted"
    );
}

/// `--rules` is a global flag on `Cli`, so it is accepted regardless of the
/// subcommand and is not a field on `Commands::Scan`.
#[test]
fn cli_parses_scan_with_rules() {
    let cli =
        Cli::try_parse_from(["lint-arwaky", "scan", "--rules", "AES101"]).expect("valid clap args");
    assert_eq!(cli.rules.as_deref(), Some("AES101"));
}

/// `--filter` is a backward-compat alias for the global `--rules` flag.
#[test]
fn cli_parses_scan_with_filter_alias() {
    let cli = Cli::try_parse_from(["lint-arwaky", "scan", "--filter", "AES101"])
        .expect("valid clap args");
    assert_eq!(cli.rules.as_deref(), Some("AES101"));
}

#[test]
fn cli_parses_scan_with_group() {
    for (group_value, expected) in [
        ("all", ScanGroup::All),
        ("quality", ScanGroup::Quality),
        ("role", ScanGroup::Role),
        ("import", ScanGroup::Import),
        ("naming", ScanGroup::Naming),
        ("orphan", ScanGroup::Orphan),
        ("external", ScanGroup::External),
        ("doc", ScanGroup::Doc),
        ("structure", ScanGroup::Structure),
    ] {
        let cli = Cli::try_parse_from(["lint-arwaky", "scan", "--group", group_value])
            .unwrap_or_else(|e| panic!("scan --group {group_value} must parse: {e}"));
        match cli.command {
            Commands::Scan { group, .. } => {
                assert_eq!(group, Some(expected), "group {group_value}");
            }
            other => panic!("expected Scan, got {other:?}"),
        }
    }
}

#[test]
fn cli_parses_check_with_group() {
    let cli = Cli::try_parse_from(["lint-arwaky", "check", "--group", "quality"])
        .expect("valid clap args");
    match cli.command {
        Commands::Scan { group, .. } => {
            assert_eq!(group, Some(ScanGroup::Quality));
        }
        other => panic!("expected Scan, got {other:?}"),
    }
}

#[test]
fn cli_rejects_invalid_group_value() {
    let result = Cli::try_parse_from(["lint-arwaky", "scan", "--group", "unknown_group"]);
    assert!(
        result.is_err(),
        "unknown group value must be rejected by clap"
    );
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
    assert_eq!(catalog.len(), COMMAND_CATALOG.len());
    for (name, _, _) in COMMAND_CATALOG {
        assert!(catalog.contains_key(&shared_cli_commands::ActionName::from(*name)));
    }
    let check = catalog.get(&shared_cli_commands::ActionName::from("check"));
    assert!(check.is_some());
    assert!(!check.unwrap().example.value.is_empty());
}

#[test]
fn command_catalog_does_not_advertise_removed_subcommands() {
    let catalog = command_catalog();
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
            !catalog.contains_key(&shared_cli_commands::ActionName::from(name)),
            "catalog must not advertise standalone '{name}' — it is a scan group, not a command"
        );
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
/// `results` and a parser `PipelineDiagnostic` for the same file in
/// `diagnostics` must count once, not twice. Parser diagnostics for other
/// files survive.
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
    let results = vec![
        LintResult::new_arch(
            "src/surface.rs",
            1,
            "AES201",
            Severity::CRITICAL,
            "surface -> capabilities import forbidden",
        ),
        parse_result,
    ];
    let dup_diagnostic = PipelineDiagnostic::new(
        "parser".to_string(),
        "File skipped: parse failure — syntax error".to_string(),
        DiagnosticSeverity::Warning,
    )
    .with_file("src/bad.rs");
    let other_file_diagnostic = PipelineDiagnostic::new(
        "parser".to_string(),
        "File skipped: parse failure — unexpected token".to_string(),
        DiagnosticSeverity::Warning,
    )
    .with_file("src/worse.rs");
    let other_diagnostic = PipelineDiagnostic::new(
        "config".to_string(),
        "no config found".to_string(),
        DiagnosticSeverity::Info,
    );

    let report = ScanReport::new(
        results,
        vec![dup_diagnostic, other_file_diagnostic, other_diagnostic],
    );

    // Only the same-file duplicate is dropped; other-file and non-parser
    // diagnostics survive.
    assert_eq!(report.diagnostics.len(), 2);
    assert!(
        !report
            .diagnostics
            .iter()
            .any(|d| d.source == "parser" && d.file.as_deref() == Some("src/bad.rs"))
    );
    assert!(
        report
            .diagnostics
            .iter()
            .any(|d| d.file.as_deref() == Some("src/worse.rs"))
    );
    // Both results (incl. the PARSE_ one) still present — the count
    // reflects each finding once.
    assert_eq!(report.results.len(), 2);
    assert_eq!(report.violation_count(), 2);

    // No PARSE_ result: parser diagnostics are kept (legacy path intact).
    let no_parse = vec![LintResult::new_arch(
        "src/surface.rs",
        1,
        "AES201",
        Severity::CRITICAL,
        "bad import",
    )];
    let report2 = ScanReport::new(
        no_parse,
        vec![
            PipelineDiagnostic::new(
                "parser".to_string(),
                "File skipped: parse failure — syntax error".to_string(),
                DiagnosticSeverity::Warning,
            )
            .with_file("src/bad.rs"),
        ],
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
