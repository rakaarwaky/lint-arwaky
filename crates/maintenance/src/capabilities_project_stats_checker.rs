use shared_common::taxonomy_common_vo::{Count, Score};
use shared_common::taxonomy_path_vo::FilePath;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_maintenance::contract_maintenance_protocol::IProjectStatsProtocol;
use shared_maintenance::taxonomy_maintenance_vo::MaintenanceStatsVO;
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ProjectStatsChecker {
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Implementation ─────────────────────
impl IProjectStatsProtocol for ProjectStatsChecker {
    fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO {
        let root = &project_path.value;
        let root_path = Path::new(root);
        let mut total_files = 0u64;
        let mut test_files = 0u64;
        let mut python_files = 0u64;
        let mut rust_files = 0u64;
        let mut js_files = 0u64;
        for entry_path in self
            .io
            .read_dir_entries_as_pathbuf(root_path)
            .unwrap_or_default()
        {
            if entry_path.is_file() {
                total_files += 1;
                let name = entry_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                // test_files counts SOURCE files only (same extension set as
                // source_count) so test_ratio stays bounded within [0.0, 1.0].
                // Non-source files like test_data.json must not inflate it.
                let is_source = matches!(
                    entry_path.extension().and_then(|e| e.to_str()),
                    Some("rs" | "py" | "ts" | "js" | "jsx" | "tsx")
                );
                if is_source && (name.contains("test") || name.contains("spec")) {
                    test_files += 1;
                }
                if let Some(ext) = entry_path.extension().and_then(|e| e.to_str()) {
                    match ext {
                        "rs" => rust_files += 1,
                        "py" => python_files += 1,
                        "ts" | "js" | "jsx" | "tsx" => js_files += 1,
                        _ => {}
                    }
                }
            }
        }
        let source_count = rust_files + python_files + js_files;
        MaintenanceStatsVO {
            project_path: project_path.clone(),
            total_files: Count::new(total_files as i64),
            test_files: Count::new(test_files as i64),
            test_ratio: Score::new(if source_count > 0 {
                test_files as f64 / source_count as f64
            } else {
                0.0
            }),
            python_files: Count::new(python_files as i64),
            rust_files: Count::new(rust_files as i64),
            js_files: Count::new(js_files as i64),
        }
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl ProjectStatsChecker {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { io }
    }
}
