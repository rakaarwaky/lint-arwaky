/// python_analysis_adapter — Python analysis adapters (Complexity, Duplicate, Trends, Dependency).
use crate::contract::{ICommandExecutorPort, ILinterAdapterPort, IPathNormalizationPort};
use crate::taxonomy::{
    AdapterName, ColumnNumber, ComplianceStatus, Count, ErrorCode, FilePath, LineNumber,
    LintMessage, LintResult, LintResultList, LinterOperationError, Severity,
};
use std::path::Path;
use std::sync::Arc;

#[allow(dead_code)]
fn resolve_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if let Ok(abs_path) = std::fs::canonicalize(path_str) {
        let mut current = if abs_path.is_file() {
            abs_path.parent().unwrap_or(Path::new(".")).to_path_buf()
        } else {
            abs_path
        };
        for _ in 0..10 {
            if current.join("lint_arwaky.config.yaml").is_file()
                || current.join(".git").is_dir()
                || current.join("pyproject.toml").is_file()
            {
                return FilePath::new(current.to_string_lossy().to_string());
            }
            if let Some(parent) = current.parent() {
                if parent == current {
                    break;
                }
                current = parent.to_path_buf();
            } else {
                break;
            }
        }
    }
    FilePath::new(".")
}

pub struct ComplexityAdapter {
    _executor: Arc<dyn ICommandExecutorPort>,
    _path_norm: Arc<dyn IPathNormalizationPort>,
    _bin_path: Option<FilePath>,
    _threshold: Count,
}

impl ComplexityAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
        bin_path: Option<FilePath>,
        threshold: Count,
    ) -> Self {
        Self {
            _executor: executor,
            _path_norm: path_norm,
            _bin_path: bin_path,
            _threshold: threshold,
        }
    }
}

#[async_trait::async_trait]
impl ILinterAdapterPort for ComplexityAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::new("radon")
    }
    async fn scan(&self, _path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        Ok(LintResultList::default())
    }
    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

pub struct DuplicateAdapter {
    _executor: Arc<dyn ICommandExecutorPort>,
    _path_norm: Arc<dyn IPathNormalizationPort>,
    _bin_path: Option<FilePath>,
}

impl DuplicateAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            _executor: executor,
            _path_norm: path_norm,
            _bin_path: bin_path,
        }
    }
}

#[async_trait::async_trait]
impl ILinterAdapterPort for DuplicateAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::new("duplicates")
    }
    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let abs_path = std::path::Path::new(&path.value);
        if abs_path.is_file() {
        } else if abs_path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(abs_path) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if let Some(ext) = p.extension() {
                        if ext == "py" || ext == "js" || ext == "ts" {
                            if let Ok(content) = std::fs::read_to_string(&p) {
                                let line_count = content.lines().count();
                                if line_count > 500 {
                                    results.push(LintResult {
                                        file: FilePath::new(p.to_string_lossy().to_string()),
                                        line: LineNumber::new(1),
                                        column: ColumnNumber::new(0),
                                        code: ErrorCode::new("DUPE001"),
                                        message: LintMessage::new(format!(
                                            "File exceeds 500 lines ({}); potential duplication.",
                                            line_count
                                        )),
                                        source: self.name(),
                                        severity: Severity::LOW,
                                        enclosing_scope: Default::default(),
                                        related_locations: Default::default(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(LintResultList::new(results))
    }
    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

pub struct TrendsAdapter {
    _executor: Arc<dyn ICommandExecutorPort>,
    _path_norm: Arc<dyn IPathNormalizationPort>,
    _history_file: FilePath,
}

impl TrendsAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
        history_file: FilePath,
    ) -> Self {
        Self {
            _executor: executor,
            _path_norm: path_norm,
            _history_file: history_file,
        }
    }
}

#[async_trait::async_trait]
impl ILinterAdapterPort for TrendsAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::new("trends")
    }
    async fn scan(&self, _path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        Ok(LintResultList::default())
    }
    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

pub struct DependencyAdapter {
    _executor: Arc<dyn ICommandExecutorPort>,
    _path_norm: Arc<dyn IPathNormalizationPort>,
    _bin_path: Option<FilePath>,
}

impl DependencyAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            _executor: executor,
            _path_norm: path_norm,
            _bin_path: bin_path,
        }
    }
}

#[async_trait::async_trait]
impl ILinterAdapterPort for DependencyAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::new("pip-audit")
    }
    async fn scan(&self, _path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        Ok(LintResultList::default())
    }
    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}
