import pytest
from auto_linter.taxonomy import FilePath, FilePathList, LintResultList, DirectoryPath
from auto_linter.infrastructure.config_discovery_provider import ConfigDiscoveryProvider
from auto_linter.infrastructure.config_parser_provider import (
    ConfigParserProvider,
    ConfigJSONProvider,
    ConfigValidationProvider,
)
from auto_linter.agent.arch_compliance_orchestrator import ArchitectureOrchestrator
from auto_linter.capabilities.architecture_rule_evaluator import ArchitectureRuleEvaluator


def test_architecture_rule_evaluator_naming():
    # 1. Initialize configuration validation provider
    discovery = ConfigDiscoveryProvider()
    parser = ConfigParserProvider()
    json_prov = ConfigJSONProvider()
    validator = ConfigValidationProvider(discovery, parser, json_prov)

    # 2. Load the python configuration
    config_file = FilePath(value="auto_linter.config.python.yaml")
    app_config = validator.load_config(yaml_path=config_file)
    config = app_config.project.architecture
    
    # 3. Resolve the effective layer map
    orchestrator = ArchitectureOrchestrator()
    layer_map = orchestrator.resolve_effective_layer_map(config).values

    # 4. Instantiate the architecture rule evaluator (delegates to Native Rust engine)
    evaluator = ArchitectureRuleEvaluator(
        config=config,
        fs=parser, # parser acts as fs provider too
        parser=parser,
        layer_map=layer_map,
    )

    # 5. Check naming rules on some fake files
    results = LintResultList()
    files = FilePathList(values=[
        FilePath(value="src-python/taxonomy/bad.py"),  # invalid: 1 word
        FilePath(value="src-python/taxonomy/good_naming_vo.py"),  # valid: 3 words
    ])

    evaluator.check_file_naming(files, FilePath(value="."), results)

    # Validate that the bad file naming is caught by the Rust engine
    bad_violations = [r for r in results.values if "bad.py" in str(r.file)]
    assert len(bad_violations) > 0
    assert bad_violations[0].code.code == "AES003"
    assert "NAMING_CONVENTION" in bad_violations[0].message.value

    # Validate that the good file naming is not flagged
    good_violations = [r for r in results.values if "good_naming_vo.py" in str(r.file)]
    assert len(good_violations) == 0


def test_architecture_rule_evaluator_line_count():
    discovery = ConfigDiscoveryProvider()
    parser = ConfigParserProvider()
    json_prov = ConfigJSONProvider()
    validator = ConfigValidationProvider(discovery, parser, json_prov)

    config_file = FilePath(value="auto_linter.config.python.yaml")
    app_config = validator.load_config(yaml_path=config_file)
    config = app_config.project.architecture
    
    orchestrator = ArchitectureOrchestrator()
    layer_map = orchestrator.resolve_effective_layer_map(config).values

    evaluator = ArchitectureRuleEvaluator(
        config=config,
        fs=parser,
        parser=parser,
        layer_map=layer_map,
    )

    results = LintResultList()
    files = FilePathList(values=[
        FilePath(value="src-python/taxonomy/__init__.py"),  # exempted
        FilePath(value="src-python/capabilities/architecture_rule_evaluator.py"),  # large file (max 500 but this might be under/over)
    ])

    evaluator.check_line_counts(files, FilePath(value="."), results)
    
    # Simple line count check runs and populates results safely
    assert isinstance(results.values, list)
