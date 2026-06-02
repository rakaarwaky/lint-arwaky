from abc import ABC, abstractmethod

from ..taxonomy import (
    AdapterName,
    Count,
    FilePath,
    FilePathList,
    LayerNameVO,
    LintResultList,
    ResponseDataList,
    Score,
    SuccessStatus,
)


class IArchRuleEngineProtocol(ABC):
    @abstractmethod
    def check_file_naming(
        self, files: FilePathList, root_dir: FilePath, results: LintResultList
    ): ...
    @abstractmethod
    def check_domain_suffixes(
        self, files: FilePathList, root_dir: FilePath, results: LintResultList
    ): ...
    @abstractmethod
    def check_layer_internal_rules(
        self, files: FilePathList, root_dir: FilePath, results: LintResultList
    ): ...
    @abstractmethod
    def check_line_counts(
        self, files: FilePathList, root_dir: FilePath, results: LintResultList
    ): ...
    @abstractmethod
    def check_no_bypass_comments(
        self, files: FilePathList, results: LintResultList
    ): ...
    @abstractmethod
    def check_unused_mandatory_imports(
        self, files: FilePathList, results: LintResultList
    ): ...
    @abstractmethod
    def check_mandatory_class_definition(
        self, files: FilePathList, root_dir: FilePath, results: LintResultList
    ): ...
    @abstractmethod
    def detect_layer(
        self, file_path: FilePath, root_dir: FilePath
    ) -> LayerNameVO | None: ...
    @abstractmethod
    def detect_module_layer(self, module_path: FilePath) -> LayerNameVO | None: ...


class IConfigRulesProtocol(ABC):
    @abstractmethod
    def is_adapter_enabled(self, adapter_name: AdapterName) -> SuccessStatus: ...
    @abstractmethod
    def validate_thresholds(self) -> SuccessStatus: ...


class IMetricAnalyzerProtocol(ABC):
    @abstractmethod
    def analyze_complexity(
        self, raw_data: ResponseDataList, threshold: Count
    ) -> LintResultList: ...
    @abstractmethod
    def analyze_file_size(
        self, file_path: FilePath, line_count: Count, limit: Count
    ) -> LintResultList: ...
    @abstractmethod
    def analyze_quality_trend(
        self, current_score: Score, previous_score: Score
    ) -> LintResultList: ...
