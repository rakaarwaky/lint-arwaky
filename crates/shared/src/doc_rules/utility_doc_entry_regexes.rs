// PURPOSE: entry-heading regexes for AES606 template conformance.
//
// The five item-heading patterns one doc's entry tables are walked with:
// `FR-` requirement entries (FRD), `PR-` entries (PRD / ROADMAP / root
// BACKLOG), `DT-` data entries (DATA), `DS-` design entries (DESIGN), and
// the family-qualified `<ID>:` entries of a member BACKLOG. Each compiles
// once and is shared by every capability that walks that doc's entries, so
// the accepted ID shapes cannot drift between them.

use std::sync::OnceLock;

/// `FR-<FEATURE>-NNN:` requirement entry heading (FRD entries carry
/// `| Aspect | Detail |` tables and `####` aspect subsections).
pub fn fr_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"^FR-[A-Za-z0-9_]+-\d+:").ok())
        .as_ref()
}

/// `PR-<PROJECT>-NNN:` item heading (PRD `Project Requirements` and ROADMAP
/// `Roadmap Status` entries).
pub fn pr_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"^PR-[A-Za-z0-9_]+-\d+:").ok())
        .as_ref()
}

/// `DT-<DATA>-NNN:` item heading (DATA `Data Requirements` entries).
pub fn dt_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"^DT-[A-Za-z0-9_]+-\d+:").ok())
        .as_ref()
}

/// `DS-<DESIGN>-NNN:` item heading (DESIGN `Design Requirements` entries).
pub fn ds_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"^DS-[A-Za-z0-9_]+-\d+:").ok())
        .as_ref()
}

/// `<ID>: <title>` single-qualifier heading (BACKLOG `Backlog Status`
/// entries). Matches the same ID families FRD/DESIGN/DATA/PRD use
/// (FR / DS / DT / PR), so a BACKLOG entry cites the same ID its spec doc
/// declares.
pub fn backlog_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(r"^(FR|DS|DT|PR)-[A-Za-z0-9_]+-\d+:\s*$").ok())
        .as_ref()
}
