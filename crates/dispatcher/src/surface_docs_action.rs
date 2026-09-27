// PURPOSE: Doc rules audit business logic, no formatting.
//
// Data Flow:
//   CLI → collect_docs → doc_orchestrator.execute(DocRequest::AuditAll) → findings
//
// The doc-rules crate audits Markdown documents only. Unlike the source-code
// rule crates it does not consume a file index: the document chain is a fixed
// set of well-known files, so the capability walks the workspace itself.
use std::sync::Arc;

use shared::doc_rules::IDocRunnerAggregate;
use shared::doc_rules::taxonomy_doc_request::DocRequest;
use shared::doc_rules::taxonomy_doc_response::DocResponse;

/// Run the doc invariant audit over *root*, returning the findings.
pub fn collect_docs(
    root: &str,
    doc_orchestrator: Arc<dyn IDocRunnerAggregate>,
) -> Result<Vec<String>, String> {
    if !std::path::Path::new(root).is_dir() {
        return Err(format!("Error: path '{root}' does not exist"));
    }
    let DocResponse::Findings { findings } = doc_orchestrator.execute(DocRequest::audit_all(root));
    Ok(findings
        .into_iter()
        .map(|finding| {
            format!(
                "{} {}: {}: {}",
                finding.code, finding.violation_type, finding.doc, finding.message
            )
        })
        .collect())
}
