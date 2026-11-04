// PURPOSE: entry-heading regexes for AESD06 template conformance.
//
// The five item-heading patterns one doc's entry tables are walked with:
// `FR-` requirement entries (FRD), `PR-` entries (PRD / ROADMAP / root
// BACKLOG), `DT-` data entries (DATA), `DS-` design entries (DESIGN), and
// the family-qualified `<ID>:` entries of a member BACKLOG. Each compiles
// once and is shared by every capability that walks that doc's entries, so
// the accepted ID shapes cannot drift between them.

use crate::taxonomy_doc_rules_constant as consts;
use std::sync::OnceLock;

/// `FR-<FEATURE>-NNN:` requirement entry heading (FRD entries carry
/// `| Aspect | Detail |` tables and `####` aspect subsections).
pub fn fr_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(consts::FR_ENTRY_PATTERN).ok())
        .as_ref()
}

/// `PR-<PROJECT>-NNN:` item heading (PRD `Project Requirements` and ROADMAP
/// `Roadmap Status` entries).
pub fn pr_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(consts::PR_ENTRY_PATTERN).ok())
        .as_ref()
}

/// `DT-<DATA>-NNN:` item heading (DATA `Data Requirements` entries).
pub fn dt_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(consts::DT_ENTRY_PATTERN).ok())
        .as_ref()
}

/// `DS-<DESIGN>-NNN:` item heading (DESIGN `Design Requirements` entries).
pub fn ds_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(consts::DS_ENTRY_PATTERN).ok())
        .as_ref()
}

/// `<ID>: <title>` single-qualifier heading (BACKLOG `Backlog Status`
/// entries). Matches the same ID families FRD/DESIGN/DATA/PRD use
/// (FR / DS / DT / PR), so a BACKLOG entry cites the same ID its spec doc
/// declares.
pub fn backlog_entry_re() -> Option<&'static regex::Regex> {
    static PAT: OnceLock<Option<regex::Regex>> = OnceLock::new();
    PAT.get_or_init(|| regex::Regex::new(consts::BACKLOG_ENTRY_PATTERN).ok())
        .as_ref()
}
