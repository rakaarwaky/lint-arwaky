// PURPOSE: IRuleEngineProtocol — protocol trait for cross-project rule enforcement
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_common_vo::ResponseDataList;
use crate::shared_common::taxonomy_common_vo::Score;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

pub trait IArchRuleEngineProtocol: Send + Sync {
    fn check_file_naming(
        &self,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_domain_suffixes(
        &self,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_layer_internal_rules(
        &self,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_line_counts(
        &self,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_no_bypass_comments(&self, files: &FilePathList, results: &mut LintResultList);
    fn check_unused_mandatory_imports(&self, files: &FilePathList, results: &mut LintResultList);
    fn check_mandatory_class_definition(
        &self,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn detect_layer(&self, file_path: &FilePath, root_dir: &FilePath) -> Option<LayerNameVO>;
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO>;
}

pub trait IConfigRulesProtocol: Send + Sync {
    fn is_adapter_enabled(&self, adapter_name: &AdapterName) -> SuccessStatus;
    fn validate_thresholds(&self) -> SuccessStatus;
}

pub trait IMetricAnalyzerProtocol: Send + Sync {
    fn analyze_complexity(&self, raw_data: &ResponseDataList, threshold: Count) -> LintResultList;
    fn analyze_file_size(
        &self,
        file_path: &FilePath,
        line_count: Count,
        limit: Count,
    ) -> LintResultList;
    fn analyze_quality_trend(
        &self,
        current_score: &Score,
        previous_score: &Score,
    ) -> LintResultList;
}
