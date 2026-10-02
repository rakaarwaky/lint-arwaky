// PURPOSE: external-lint constants — CLI names and flags the external adapters shell out to
// Used by: external-lint (MarkdownLintAdapter) to probe both markdownlint CLI variants

/// The autofix flag, spelled identically by both markdownlint CLI variants.
pub const MARKDOWNLINT_FIX_FLAG: &str = "--fix";

/// The markdownlint CLI variants to probe, in order. `markdownlint-cli` speaks
/// JSON under `--json`; `markdownlint-cli2` has no JSON mode and writes text.
pub const MARKDOWNLINT_CLI_VARIANTS: [&str; 2] = ["markdownlint-cli", "markdownlint-cli2"];

/// Extensions markdownlint lints by default.
pub const MARKDOWNLINT_EXTENSIONS: [&str; 2] = ["md", "markdown"];
