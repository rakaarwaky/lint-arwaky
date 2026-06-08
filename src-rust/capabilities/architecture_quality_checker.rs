use crate::contract::architecture_rule_protocol::ICodeQualityProtocol;
use crate::contract::{IAnalyzer, IArchRuleProtocol, IFileSystemPort, ISourceParserPort};
use crate::taxonomy::{
    AdapterName, ColumnNumber, CustomMessageVO, ErrorCode, ErrorMessage, FilePath, FilePathList,
    Identity, LayerNameVO, LineNumber, LintMessage, LintResult, LintResultList, ModuleName,
    PatternList, Severity,
};
/// Architecture rule checking: bypass comments and unused imports.
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
        _custom_messages: Option<&[CustomMessageVO]>,
    ) {
        let forbidden = match forbidden_words {
            Some(fw) if !fw.values.is_empty() => fw.values,
            _ => return,
        };
        let content = match fs.read_text(&file_path).await {
            Ok(c) => c.value,
            Err(_) => return,
        };
        let base_msg = violation_message.map(|m| m.value).unwrap_or_else(|| {
            "STOP CHEATING! You are strictly forbidden from using bypass comments.".to_string()
        });
        for (i, line) in content.lines().enumerate() {
            if let Some((col, final_msg)) = self.find_bypass_matches(line, &forbidden, &base_msg) {
                results.push(LintResult {
                    file: file_path.clone(),
                    line: LineNumber::new((i + 1) as i64),
                    column: ColumnNumber::new(col),
                    code: ErrorCode::raw("AES014"),
                    message: LintMessage::new(final_msg),
                    source: Some(AdapterName::raw("architecture")),
                    severity: Severity::CRITICAL,
                    enclosing_scope: None,
                    related_locations: crate::taxonomy::LocationList::default(),
                });
            }
        }
    }

    pub async fn check_unused_mandatory_imports(
        &self,
        _files: &[FilePath],
        _parser: &dyn crate::contract::ISourceParserPort,
        _results: &mut LintResultList,
        _violation_message: Option<&ErrorMessage>,
        _mandatory_imports: Option<&PatternList>,
    ) {
        // Placeholder — full implementation requires AST integration
    }

    pub async fn check_dead_inheritance_bypass(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &[FilePath],
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        // Placeholder — full implementation requires AST integration
    }

    pub async fn check_forbidden_inheritance(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &[FilePath],
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        // Placeholder — full implementation requires AST integration
    }
}

impl IArchRuleProtocol for CodeQualityRuleChecker {
    fn rule_name(&self) -> Identity {
        self.rule_name()
    }
}

#[async_trait]
impl ICodeQualityProtocol for CodeQualityRuleChecker {
    async fn check_no_bypass_comments(
        &self,
        file_path: &FilePath,
        fs: &dyn IFileSystemPort,
        results: &mut LintResultList,
        forbidden_words: Option<&PatternList>,
        violation_message: Option<&ErrorMessage>,
        custom_messages: Option<&[CustomMessageVO]>,
    ) {
        self.check_no_bypass_comments(
            file_path.clone(),
            fs,
            results,
            forbidden_words.cloned(),
            violation_message.cloned(),
            custom_messages,
        )
        .await;
    }

    async fn check_unused_mandatory_imports(
        &self,
        _files: &FilePathList,
        _parser: &dyn ISourceParserPort,
        _results: &mut LintResultList,
        _violation_message: Option<&ErrorMessage>,
        _mandatory_imports: Option<&PatternList>,
        _layer_resolver: Option<&(dyn Fn(&ModuleName) -> Option<LayerNameVO> + Sync)>,
    ) {
    }

    async fn check_dead_inheritance_bypass(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
    }

    async fn check_forbidden_inheritance(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
    }
}
