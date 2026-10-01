// PURPOSE: CodeAnalysisResponse — response payload for the code_analysis aggregate

use crate::taxonomy_code_analysis_vo::CodeAnalysisRuleVO;
use shared_common::taxonomy_common_vo::{BooleanVO, Score};
use shared_common::taxonomy_display_content_vo::DisplayContent;
use shared_common::taxonomy_lint_result_vo::LintResult;

pub enum CodeAnalysisResponse {
    Analysis { violations: Vec<LintResult> },
    Score { score: Score },
    Report { content: DisplayContent },
    Critical { is_critical: BooleanVO },
    Rules { rules: Vec<CodeAnalysisRuleVO> },
}

impl CodeAnalysisResponse {
    pub fn into_violations(self) -> Vec<LintResult> {
        match self {
            Self::Analysis { violations } => violations,
            _ => Vec::new(),
        }
    }

    pub fn into_score(self) -> Score {
        match self {
            Self::Score { score } => score,
            _ => Score::default(),
        }
    }

    pub fn into_content(self) -> DisplayContent {
        match self {
            Self::Report { content } => content,
            _ => DisplayContent::default(),
        }
    }

    pub fn into_is_critical(self) -> BooleanVO {
        match self {
            Self::Critical { is_critical } => is_critical,
            _ => BooleanVO::default(),
        }
    }

    pub fn into_rules(self) -> Vec<CodeAnalysisRuleVO> {
        match self {
            Self::Rules { rules } => rules,
            _ => Vec::new(),
        }
    }
}
