use crate::multi_project::contract_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use crate::multi_project::taxonomy_summary_vo::AggregatedResults;
use crate::multi_project::taxonomy_summary_vo::ProjectResult;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_common_vo::PatternList;
use crate::shared_common::taxonomy_common_vo::Score;
use crate::shared_common::taxonomy_layer_vo::Identity;
use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

pub struct MultiProjectOrchestrator {}

#[async_trait]
impl MultiProjectOrchestratorAggregate for MultiProjectOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        None
    }

    async fn analyze_project(&self, path: &FilePath) -> ProjectResult {
        ProjectResult {
            path: path.clone(),
            score: Score::new(100.0),
            is_passing: ComplianceStatus::new(true),
            issues: Vec::new(),
            adapters: PatternList::new(Vec::<String>::new()),
            error: ErrorMessage::default(),
        }
    }

    async fn scan_all_projects(
        &self,
        paths: &FilePathList,
        _max_concurrency: Count,
    ) -> AggregatedResults {
        let mut results = Vec::new();
        for path in &paths.values {
            let result = self.analyze_project(path).await;
            results.push(result);
        }
        self.aggregate_results(results)
    }

    fn load_config(config_path: Option<&FilePath>) -> FilePathList {
        let path_str = match config_path {
            Some(fp) => &fp.value,
            None => "",
        };
        let path = std::path::Path::new(path_str);
        if !path.exists() {
            return FilePathList::new(Vec::new());
        }
        if let Some(ext) = path.extension() {
            if ext == "json" {
                if let Ok(content) = std::fs::read_to_string(path) {
                    if let Ok(data) = serde_json::from_str::<
                        std::collections::HashMap<String, Vec<String>>,
                    >(&content)
                    {
                        let projects = data.get("projects").cloned().unwrap_or_default();
                        return FilePathList::new(
                            projects
                                .into_iter()
                                .map(|p| FilePath::new(p).unwrap_or_default())
                                .collect(),
                        );
                    }
                }
            }
        }
        FilePathList::new(Vec::new())
    }

    fn find_projects(root: &FilePath, config_name: &Identity) -> FilePathList {
        let root_path = std::path::Path::new(&root.value);
        let mut projects = Vec::new();
        fn visit_dirs(dir: &std::path::Path, config_name: &str, projects: &mut Vec<FilePath>) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.is_dir() {
                        visit_dirs(&p, config_name, projects);
                    } else if p.file_name().is_some_and(|n| n == config_name) {
                        if let Some(parent) = p.parent() {
                            if let Ok(fp) = FilePath::new(parent.to_string_lossy().to_string()) {
                                projects.push(fp);
                            }
                        }
                    }
                }
            }
        }
        visit_dirs(root_path, config_name.value(), &mut projects);
        FilePathList::new(projects)
    }
}

impl Default for MultiProjectOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiProjectOrchestrator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn aggregate_results(&self, projects: Vec<ProjectResult>) -> AggregatedResults {
        let total = projects.len();
        let passing = projects.iter().filter(|p| p.is_passing.value).count();
        let scores: Vec<f64> = projects
            .iter()
            .map(|p| p.score.value)
            .filter(|s| *s > 0.0)
            .collect();
        let avg_score = if scores.is_empty() {
            0.0
        } else {
            scores.iter().sum::<f64>() / scores.len() as f64
        };

        AggregatedResults {
            projects,
            total_projects: Count::new(total as i64),
            passing_projects: Count::new(passing as i64),
            failing_projects: Count::new((total - passing) as i64),
            average_score: Score::new(avg_score),
        }
    }
}
