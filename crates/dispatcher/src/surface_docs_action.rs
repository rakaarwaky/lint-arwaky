// PURPOSE: Doc rules audit business logic, no formatting.
//
// Data Flow:
//   CLI → collect_docs → doc_orchestrator.execute(DocRequest::AuditAll) → findings
//
// The doc-rules crate audits Markdown documents only. Unlike the source-code
// rule crates it does not consume a file index: the document chain is a fixed
// set of well-known files, so the capability walks the workspace itself.
use std::sync::Arc;

use shared::common::ColumnNumber;
use shared::common::ErrorCode;
use shared::common::FilePath;
use shared::common::LineNumber;
use shared::common::LintMessage;
use shared::common::Severity;
use shared::common::ViolationItem;
use shared::doc_rules::IDocRunnerAggregate;
use shared::doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared::doc_rules::taxonomy_doc_rules_response::DocResponse;

/// A structured doc finding carrying the fields every output format needs:
/// rule id, target document, violation type, detail message, and a human-
/// readable "file:line" location string that JSON/SARIF/JUnit emitters embed.
/// `severity` is fixed to HIGH: doc invariants are architectural policy gaps.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocsFinding {
    /// The invariant code that fired, e.g. `AES601`.
    pub code: String,
    /// The document this finding belongs to (may be empty for global findings).
    pub doc: String,
    /// A machine-parseable violation subtype (e.g. `missing-frd`).
    pub violation_type: String,
    /// Human-readable detail about what drifted.
    pub message: String,
    /// "doc" or "doc:line" when the checker reports a line offset, else "doc".
    pub location: String,
    /// Severity bucket used for JSON's `"severity"` and SARIF's `"level"`.
    pub severity: Severity,
}

impl DocsFinding {
    /// Human-readable "file:line [code] violation_type — message" summary.
    pub fn summary(&self) -> String {
        format!(
            "{} [{}] {} — {}",
            self.location, self.code, self.violation_type, self.message
        )
    }
}

/// Run the doc invariant audit over *root*, returning structured findings.
pub fn collect_docs(
    root: &str,
    doc_orchestrator: Arc<dyn IDocRunnerAggregate>,
) -> Result<Vec<DocsFinding>, String> {
    if !std::path::Path::new(root).is_dir() {
        return Err(format!("Error: path '{root}' does not exist"));
    }
    let DocResponse::Findings { findings } = doc_orchestrator.execute(DocRequest::audit_all(root));
    Ok(findings
        .into_iter()
        .map(|finding| {
            // Doc checkers report some findings on a line offset inside the
            // document (the message carries it, e.g. "line N heading ...");
            // others are document-level and fall back to the doc path.
            let doc_path = finding.doc.clone();
            let location = finding
                .message
                .find("line ")
                .and_then(|idx| {
                    let n = finding.message[idx + 5..]
                        .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>();
                    if n.is_empty() {
                        None
                    } else {
                        Some(format!("{doc_path}:{n}"))
                    }
                })
                .unwrap_or(doc_path);
            DocsFinding {
                code: finding.code.to_string(),
                doc: finding.doc.clone(),
                violation_type: finding.violation_type.to_string(),
                message: finding.message.clone(),
                location,
                severity: Severity::HIGH,
            }
        })
        .collect())
}

/// Convert structured doc findings into generic `ViolationItem`s so the
/// existing `utility_output_text_formatter::output_violations` emitters
/// (text/json/sarif/junit) can render them without a special path.
pub fn docs_findings_to_violations(findings: &[DocsFinding]) -> Vec<ViolationItem> {
    findings
        .iter()
        .map(|f| {
            let file = FilePath::new(f.doc.clone()).unwrap_or_default();
            let line = match f.location.rsplit_once(':') {
                Some((_, num)) => num
                    .parse::<i64>()
                    .map(LineNumber::new)
                    .unwrap_or_else(|_| LineNumber::new(0)),
                None => LineNumber::new(0),
            };
            ViolationItem {
                code: ErrorCode::raw(f.code.clone()),
                file,
                line,
                column: ColumnNumber::new(0),
                message: LintMessage::new(f.message.clone()),
                severity: f.severity.clone(),
            }
        })
        .collect()
}
