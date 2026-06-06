use crate::taxonomy::FilePath;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaintenanceStatsVO {
    pub project_path: FilePath,
    pub total_files: i64,
    pub test_files: i64,
    pub test_ratio: f64,
    pub python_files: i64,
}

impl MaintenanceStatsVO {
    pub fn new(project_path: FilePath, total_files: i64, test_files: i64, test_ratio: f64, python_files: i64) -> Self {
        Self { project_path, total_files, test_files, test_ratio, python_files }
    }
}

impl std::fmt::Display for MaintenanceStatsVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaintenanceStats({}: {} files, {} test, {:.1}%)", self.project_path, self.total_files, self.test_files, self.test_ratio * 100.0)
    }
}
