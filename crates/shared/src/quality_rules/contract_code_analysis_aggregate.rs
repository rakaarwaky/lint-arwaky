// PURPOSE: ICodeAnalysisAggregate — aggregate trait for code quality analysis (AES301–AES305)
use crate::quality_rules::taxonomy_code_analysis_request_vo::{
    CodeAnalysisRequest, CodeAnalysisResponse,
};

pub trait ICodeAnalysisAggregate: Send + Sync {
    fn execute(&self, request: CodeAnalysisRequest) -> CodeAnalysisResponse;
}
