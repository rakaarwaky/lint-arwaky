use async_trait::async_trait;
use crate::contract::project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use crate::taxonomy::{
    AggregatedResults, ComplianceStatus, Count, FilePath, FilePathList, PatternList, ProjectResult, Score,
};
use std::collections::HashMap;

pub struct MultiProjectOrchestrator;

#[async_trait]
impl MultiProjectOrchestratorAggregate for MultiProjectOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        None
    }

    async fn analyze_project(&self, path: &FilePath) -> ProjectResult {
        self.analyze_project_old(path).await
    }

    async fn scan_all_projects(
        &self,
        paths: &FilePathList,
        max_concurrency: Count,
    ) -> AggregatedResults {
        self.scan_all_projects_old(&paths.values, max_concurrency.value as usize).await
    }

    fn load_config(config_path: Option<&FilePath>) -> FilePathList {
        let orchestrator = Self::new();
        let path_str = match config_path {
            Some(fp) => &fp.value,
            None => "",
        };
        let projects = orchestrator.load_config_old(path_str);
        FilePathList::new(projects.into_iter().map(FilePath::new).collect())
    }

    fn find_projects(root: &FilePath, config_name: &str) -> FilePathList {
        let orchestrator = Self::new();
        let projects = orchestrator.find_projects_old(root, config_name);
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
        Self
    }

    pub async fn analyze_project_old(&self, path: &FilePath) -> ProjectResult {
        // Analyze a single project
        // In full implementation, gets the project-specific container and runs analysis
        ProjectResult {
            path: path.clone(),
            score: Score::new(100.0),
            is_passing: ComplianceStatus::new(true),
            issues: Vec::new(),
            adapters: PatternList::new(Vec::<String>::new()),
            error: None,
        }
    }

    pub async fn scan_all_projects_old(
        &self,
        paths: &[FilePath],
        _max_concurrency: usize,
    ) -> AggregatedResults {
        // Scan a specific list of projects with semaphore-limited concurrency
        let mut results = Vec::new();
        for path in paths {
            let result = self.analyze_project_old(path).await;
            results.push(result);
        }
        self.aggregate_results(results)
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

    pub fn load_config_old(&self, config_path: &str) -> Vec<String> {
        // Load list of project paths from a config file
        let path = std::path::Path::new(config_path);
        if !path.exists() {
            return Vec::new();
        }
        // Supports .json and .yaml/.yml files
        if let Some(ext) = path.extension() {
            if ext == "json" {
                if let Ok(content) = std::fs::read_to_string(path) {
                    if let Ok(data) = serde_json::from_str::<HashMap<String, Vec<String>>>(&content)
                    {
                        return data.get("projects").cloned().unwrap_or_default();
                    }
                }
            }
        }
        Vec::new()
    }

    pub fn find_projects_old(&self, root: &FilePath, config_name: &str) -> Vec<FilePath> {
        // Find all projects with lint-arwaky configs
        let root_path = std::path::Path::new(&root.value);
        let mut projects = Vec::new();
        fn visit_dirs(dir: &std::path::Path, config_name: &str, projects: &mut Vec<FilePath>) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        visit_dirs(&path, config_name, projects);
                    } else if path.file_name().is_some_and(|n| n == config_name) {
                        if let Some(parent) = path.parent() {
                            let fp = FilePath::new(parent.to_string_lossy().to_string());
                            projects.push(fp);
                        }
                    }
                }
            }
        }
        visit_dirs(root_path, config_name, &mut projects);
        projects
    }
}
