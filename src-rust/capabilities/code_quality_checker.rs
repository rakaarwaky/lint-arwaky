/// Architecture rule checking: bypass comments and unused imports.
use crate::contract::*;
use crate::taxonomy::*;
use async_trait::async_trait;

pub struct CodeQualityRuleChecker;

impl CodeQualityRuleChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn rule_name(&self) -> Identity {
        Identity::new("code_quality")
    }

    fn find_bypass_matches(
        &self,
        line: &str,
        forbidden: &[String],
        base_msg: &str,
    ) -> Option<(i64, String)> {
        let lower_line = line.to_lowercase();
        for bypass in forbidden {
            if lower_line.contains(&bypass.to_lowercase()) {
                let col = line.find('#').unwrap_or(0) as i64;
                return Some((col, base_msg.to_string()));
            }
        }
        None
    }

    pub async fn check_no_bypass_comments(
        &self,
        file_path: FilePath,
        fs: &dyn IFileSystemPort,
        results: &mut LintResultList,
        forbidden_words: Option<PatternList>,
        violation_message: Option<ErrorMessage>,
    ) {
        let forbidden = match forbidden_words {
            Some(fw) if !fw.values.is_empty() => fw.values,
            _ => return,
        };

        let content = match fs.read_text(file_path.clone()).await {
            Ok(c) => c.value,
            Err(_) => return,
        };

        let base_msg = violation_message
            .map(|m| m.value)
            .unwrap_or_else(|| "STOP CHEATING! You are strictly forbidden from using bypass comments.".to_string());

        for (i, line) in content.lines().enumerate() {
            if let Some((col, final_msg)) = self.find_bypass_matches(line, &forbidden, &base_msg) {
                results.append(LintResult {
                    file: file_path.clone(),
                    line: LineNumber::new((i + 1) as i64),
                    column: ColumnNumber::new(col),
                    code: ErrorCode::new("AES014"),
                    message: LintMessage::new(final_msg),
                    source: Some(AdapterName::new("architecture")),
                    severity: Severity::CRITICAL,
                    ..Default::default()
                });
            }
        }
    }
}
