// PURPOSE: external-lint constants — CLI names and flags the external adapters shell out to
// Used by: external-lint (MarkdownLintAdapter) to probe both markdownlint CLI variants

/// The autofix flag, spelled identically by both markdownlint CLI variants.
pub const MARKDOWNLINT_FIX_FLAG: &str = "--fix";

/// The markdownlint CLI variants to probe, in order. `markdownlint-cli` speaks
/// JSON under `--json`; `markdownlint-cli2` has no JSON mode and writes text.
pub const MARKDOWNLINT_CLI_VARIANTS: [&str; 2] = ["markdownlint-cli", "markdownlint-cli2"];

/// Default per-invocation scan timeout (seconds) for the markdownlint adapter.
///
/// Lower than the 60 s the other adapters hard-code: on a large tree
/// (e.g. `packages/` in a web monorepo) a runaway markdownlint walk dominates
/// the whole scan, and 30 s keeps one adapter from blocking it. A project can
/// override this per adapter through `AdapterEntry.timeout` in
/// `lint_arwaky.config.yaml`, which the surface layer forwards on the scan
/// request.
pub const MARKDOWNLINT_DEFAULT_SCAN_TIMEOUT_SECS: f64 = 30.0;

/// `ignored_rules:` patterns that fully silence the markdownlint adapter.
///
/// The orchestrator's rule filter is a case-insensitive substring match:
/// a bare `MD` entry matches every `markdownlint::MD###` code, and an entry
/// with the `markdownlint::` prefix matches every code the adapter emits.
/// Either one lets the adapter skip spawning the subprocess entirely.
pub const MARKDOWNLINT_FULL_SUPPRESSION_PREFIX: &str = "MARKDOWNLINT::";

/// Extensions markdownlint lints by default.
pub const MARKDOWNLINT_EXTENSIONS: [&str; 2] = ["md", "markdown"];
