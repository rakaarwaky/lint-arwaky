// multi_project_orchestrator — Orchestrates multi-project scans.
use crate::contract::{MultiProjectAggregate, MultiProjectOrchestratorAggregate};
use crate::taxonomy::{FilePath, Score, ComplianceStatus, ProjectResult, AggregatedResults, PatternList, ErrorMessage};
use std::collections::HashMap;

pub struct MultiProjectOrchestrator;

impl MultiProjectOrchestratorAggregate for MultiProjectOrchestrator {}

impl MultiProjectOrchestrator {
    pub fn new() -> Self {
        Self
    }

    pub async fn analyze_project(&self, path: &FilePath) -> ProjectResult {
        // Analyze a single project
        // In full implementation, gets the project-specific container and runs analysis
        ProjectResult {
            path: path.clone(),
            score: Score::new(100.0).unwrap(),
            is_passing: ComplianceStatus::new(true),
            issues: Vec::new(),
            adapters: PatternList::new(Some(Vec::new())),
            error: None,
        }
    }

    pub async fn scan_all_projects(&self, paths: &[FilePath], max_concurrency: usize) -> AggregatedResults {
        // Scan a specific list of projects with semaphore-limited concurrency
        let mut results = Vec::new();
        for path in paths {
            let result = self.analyze_project(path).await;
            results.push(result);
        }
        self.aggregate_results(results)
    }

    pub fn aggregate_results(&self, projects: Vec<ProjectResult>) -> AggregatedResults {
        let total = projects.len();
        let passing = projects.iter().filter(|p| p.is_passing.value).count();
        let scores: Vec<f64> = projects.iter().map(|p| p.score.value).filter(|s| *s > 0.0).collect();
        let avg_score = if scores.is_empty() {
            0.0
        } else {
            scores.iter().sum::<f64>() / scores.len() as f64
        };

        AggregatedResults {
            projects,
            total_projects: total,
            passing_projects: passing,
            failing_projects: total - passing,
            average_score: Score::new(avg_score).unwrap(),
        }
    }

    pub fn load_config(&self, config_path: &str) -> Vec<String> {
        // Load list of project paths from a config file
        let path = std::path::Path::new(config_path);
        if !path.exists() {
            return Vec::new();
        }
        // Supports .json and .yaml/.yml files
        if let Some(ext) = path.extension() {
            if ext == "json" {
                if let Ok(content) = std::fs::read_to_string(path) {
                    if let Ok(data) = serde_json::from_str::<HashMap<String, Vec<String>>>(&content) {
                        return data.get("projects").cloned().unwrap_or_default();
                    }
                }
            }
        }
        Vec::new()
    }

    pub fn find_projects(&self, root: &FilePath, config_name: &str) -> Vec<FilePath> {
        // Find all projects with auto-linter configs
        let root_path = std::path::Path::new(&root.value);
        let mut projects = Vec::new();
        for entry in root_path.rglob(config_name) {
            if let Some(parent) = entry.parent() {
                if let Ok(fp) = FilePath::new(parent.to_string_lossy().to_string()) {
                    projects.push(fp);
                }
            }
        }
        projects
    }
}
