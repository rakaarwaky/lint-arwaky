// PURPOSE: The output text of the naming rules — the finding template, the
// width budget, the exemption pointer, and the audit summary.
//
// Every AES101/AES102/AES103 finding is built here, so the three rules cannot
// drift into three sentence orders, three column budgets, or three ways of
// listing the legal values. The template is written down in the crate FRD;
// this file is the code that keeps that document true.
//
// Plain-language rules the emitted text follows:
//
//   - one clause per line, and the problem clause comes before the action;
//   - every line stays inside the width budget, so a finding never wraps in
//     the reader's terminal;
//   - every word the rules write is ASCII, so the text survives a terminal
//     that cannot render a Unicode dash;
//   - a file name is reproduced exactly as the developer typed it.

use crate::taxonomy_naming_audit_stats_vo::NamingAuditStats;
use crate::taxonomy_naming_finding_vo::NamingFinding;
use crate::taxonomy_naming_rules_constant::{
    RULE_CODE_NAMING_CONVENTION, RULE_CODE_SUFFIX_PREFIX, RULE_CODE_TEST_FILE_PREFIX,
};
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_violation_item_vo::ViolationItem;

/// The width budget for one rendered line of a naming finding.
///
/// Seventy-two of an eighty-column terminal: the eight spare columns hold the
/// indent a surface prints the finding under, so a finding never wraps in the
/// reader's terminal. A single word longer than the budget keeps its own line
/// rather than being cut in half — a path that cannot be split stays readable.
pub const NAMING_LINE_WIDTH: usize = 72;

/// The indent a wrapped line carries.
const HANGING_INDENT: &str = "  ";

/// The label one rule carries in the audit summary.
const AES101_LABEL: &str = "stem shape";
const AES102_LABEL: &str = "layer suffix/prefix";
const AES103_LABEL: &str = "test/bench prefix";

/// Build the three fields of one finding from their clauses.
///
/// The clause order is the template's order: the problem first, then the
/// action, then the rule. Each clause becomes one line of the rendered
/// finding; a clause longer than the width budget wraps onto continuation
/// lines indented by two spaces.
pub fn naming_finding(problem: &[&str], fix: &[&str], why: &[&str]) -> NamingFinding {
    NamingFinding::new(
        render_clauses(problem),
        render_clauses(fix),
        render_clauses(why),
    )
}

/// The clause naming the exemption mechanism of one rule.
///
/// A developer whose name is correct needs a way out that does not switch the
/// rule off for the whole repository. The exemption list is that way out, so
/// every fix names it — with the config key and the file-name form, because
/// neither is guessable.
pub fn exemption_clause(rule_code: &str, file_name: &str) -> String {
    format!(
        "If this name is intentional, add '{file_name}' to \
         architecture.rules.{rule_code}.exceptions in lint_arwaky.config.yaml."
    )
}

/// Render the summary of one naming audit.
///
/// One line per rule, so a reader can tell "your names are clean" from
/// "nothing was checked" — the two used to look the same, because both
/// produced an empty list of findings. The totals line then says how many
/// findings there were, at which level, and that the tool changed nothing.
pub fn render_naming_summary(stats: &NamingAuditStats, violations: &[ViolationItem]) -> String {
    let mut out = String::from("Naming audit:\n");
    out.push_str(&rule_line(
        RULE_CODE_NAMING_CONVENTION,
        AES101_LABEL,
        stats.aes101_inspected,
        findings_for(RULE_CODE_NAMING_CONVENTION, violations),
    ));
    out.push_str(&rule_line(
        RULE_CODE_SUFFIX_PREFIX,
        AES102_LABEL,
        stats.aes102_inspected,
        findings_for(RULE_CODE_SUFFIX_PREFIX, violations),
    ));
    out.push_str(&rule_line(
        RULE_CODE_TEST_FILE_PREFIX,
        AES103_LABEL,
        stats.aes103_inspected,
        findings_for(RULE_CODE_TEST_FILE_PREFIX, violations),
    ));
    out.push('\n');
    out.push_str(&totals_line(violations));
    out
}

/// The summary line of one rule.
fn rule_line(code: &str, label: &str, inspected: usize, findings: usize) -> String {
    let state = if findings == 0 {
        "clean".to_string()
    } else {
        plural(findings, "finding", "findings")
    };
    format!(
        "  {code}  {label:<20}  {inspected:>5} files checked  {state}\n"
    )
}

/// How many findings carry *code*.
fn findings_for(code: &str, violations: &[ViolationItem]) -> usize {
    violations.iter().filter(|v| v.code.code() == code).count()
}

/// The totals line of the audit summary.
///
/// The counts are per level, because a file that sits in the wrong folder and
/// a file with a capital letter are not the same weight, and a report that
/// shows one number for both cannot be triaged.
fn totals_line(violations: &[ViolationItem]) -> String {
    let total = violations.len();
    if total == 0 {
        return "0 findings. Nothing was renamed or moved.\n".to_string();
    }
    let mut levels: Vec<String> = Vec::new();
    for level in [
        Severity::CRITICAL,
        Severity::HIGH,
        Severity::MEDIUM,
        Severity::LOW,
        Severity::INFO,
    ] {
        let count = violations.iter().filter(|v| v.severity == level).count();
        if count > 0 {
            levels.push(format!("{count} {}", level_name(level)));
        }
    }
    format!(
        "{}: {}. Nothing was renamed or moved.\n",
        plural(total, "finding", "findings"),
        levels.join(", ")
    )
}

/// The count and the word that fits it.
///
/// "1 finding" and "2 findings": a report that says "1 findings" reads as a
/// tool that did not finish counting.
fn plural(count: usize, singular: &str, plural_form: &str) -> String {
    let word = if count == 1 { singular } else { plural_form };
    format!("{count} {word}")
}

/// The name one severity level carries in the summary line.
fn level_name(level: Severity) -> &'static str {
    match level {
        Severity::CRITICAL => "critical",
        Severity::HIGH => "high",
        Severity::MEDIUM => "medium",
        Severity::LOW => "low",
        Severity::INFO => "info",
    }
}

/// Render a list of clauses as the lines of one field.
fn render_clauses(clauses: &[&str]) -> String {
    let mut lines: Vec<String> = Vec::new();
    for clause in clauses {
        lines.extend(wrap_clause(clause));
    }
    lines.join("\n")
}

/// Wrap one clause to the width budget.
///
/// A word that does not fit starts the next line; a word longer than the whole
/// budget keeps its own line rather than being cut in half.
fn wrap_clause(clause: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    for word in clause.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if fits(&current, word) {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(std::mem::take(&mut current));
            current.push_str(HANGING_INDENT);
            current.push_str(word);
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

/// Whether *word* still fits on the line *current*.
fn fits(current: &str, word: &str) -> bool {
    current.chars().count() + 1 + word.chars().count() <= NAMING_LINE_WIDTH
}
