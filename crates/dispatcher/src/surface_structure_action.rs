// PURPOSE: Structure rules audit business logic, no formatting.
//
// Data Flow:
//   scan → collect_structure → structure_orchestrator.execute(AuditAll) → violations
//
// The structure-rules crate audits folder layout only: which layer files sit
// in which workspace member folder. It needs no file index and no AST, so it
// walks the member directories directly, exactly as the doc-rules crate walks
// the document chain.
use std::sync::Arc;

use shared_common::ViolationItem;
use shared_structure_rules::IStructureAggregate;
use shared_structure_rules::taxonomy_structure_rules_request::StructureRequest;
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;

/// Run the folder-layout audit over *root*, returning the violations.
pub fn collect_structure(
    root: &str,
    structure_orchestrator: Arc<dyn IStructureAggregate>,
) -> Result<Vec<ViolationItem>, String> {
    if !std::path::Path::new(root).is_dir() {
        return Err(format!("Error: path '{root}' does not exist"));
    }
    let StructureResponse::Findings { findings } =
        structure_orchestrator.execute(StructureRequest::audit_all(root));
    Ok(findings
        .into_iter()
        .map(|f| {
            let code = shared_common::ErrorCode::raw(f.code.clone());
            let file = shared_common::FilePath::new(f.file.clone()).unwrap_or_default();
            let message = shared_common::LintMessage::new(f.message.clone());
            ViolationItem {
                code,
                file,
                line: shared_common::LineNumber::default(),
                column: shared_common::ColumnNumber::default(),
                message,
                severity: shared_common::Severity::MEDIUM,
                violation_name: f.violation_type,
                why: String::new(),
                fix: String::new(),
            }
        })
        .collect())
}
