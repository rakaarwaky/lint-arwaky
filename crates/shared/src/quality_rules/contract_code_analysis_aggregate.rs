// PURPOSE: ICodeAnalysisAggregate — aggregate trait for code quality analysis (AES301–AES305)
use crate::taxonomy_quality_rules_request::CodeAnalysisRequest;
use crate::taxonomy_quality_rules_response::CodeAnalysisResponse;

pub trait ICodeAnalysisAggregate: Send + Sync {
    fn execute(&self, request: CodeAnalysisRequest) -> CodeAnalysisResponse;
}
