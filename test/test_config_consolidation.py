import tempfile
from pathlib import Path
import pytest

from auto_linter.infrastructure.config_discovery_provider import ConfigDiscoveryProvider
from auto_linter.infrastructure.config_parser_provider import (
    ConfigParserProvider,
    ConfigValidationProvider,
    ConfigJSONProvider,
    ConfigYamlProvider,
)
from auto_linter.taxonomy import DirectoryPath, FilePath, ProjectConfig


def test_config_discovery_and_fallback():
    discovery = ConfigDiscoveryProvider()

    # Test dominant language detection and priority ordering
    assert discovery._priority_order("rust")[0] == ("rust", "auto_linter.config.rust.yaml")
    assert discovery._priority_order("javascript")[0] == ("javascript", "auto_linter.config.javascript.yaml")
    assert discovery._priority_order("python")[0] == ("python", "auto_linter.config.python.yaml")
    assert discovery._priority_order(None) == [
        ("rust", "auto_linter.config.rust.yaml"),
        ("javascript", "auto_linter.config.javascript.yaml"),
        ("python", "auto_linter.config.python.yaml"),
    ]


def test_config_discovery_in_temp_dir():
    discovery = ConfigDiscoveryProvider()

    with tempfile.TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)
        env_file = tmp_path / ".env"
        env_file.touch()

        # Check find_env_file
        found_env = discovery.find_env_file(start=DirectoryPath(value=str(tmp_path)))
        assert found_env is not None
        assert Path(found_env.value).name == ".env"

        # Create dominant language files
        (tmp_path / "src").mkdir()
        (tmp_path / "src" / "main.rs").touch()
        (tmp_path / "src" / "helper.rs").touch()
        (tmp_path / "src" / "index.js").touch()

        # Dominant should be rust
        assert discovery._detect_dominant_language(tmp_path) == "rust"

        # Create auto_linter.config.rust.yaml
        config_file = tmp_path / "auto_linter.config.rust.yaml"
        config_file.write_text("project_name: 'test_rust'\n")

        found_yaml = discovery.find_yaml_config(start=DirectoryPath(value=str(tmp_path)))
        assert found_yaml is not None
        assert Path(found_yaml.value).name == "auto_linter.config.rust.yaml"


def test_config_validation_and_parsing():
    discovery = ConfigDiscoveryProvider()
    parser = ConfigParserProvider()
    json_prov = ConfigJSONProvider()
    validator = ConfigValidationProvider(discovery, parser, json_prov)

    with tempfile.TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)
        config_file = tmp_path / "auto_linter.config.python.yaml"
        config_file.write_text("project_name: 'test_validation_project'\n")

        # Load configuration
        app_config = validator.load_config(yaml_path=FilePath(value=str(config_file)))
        assert app_config.project.project_name == "test_validation_project"

        # Check getter
        assert validator.get_config() == app_config


def test_legacy_providers():
    json_prov = ConfigJSONProvider()
    yaml_prov = ConfigYamlProvider()

    # JSON should return defaults
    assert json_prov.load_project_config() == ProjectConfig.defaults()

    # Yaml fallback with missing file should return defaults
    assert yaml_prov.load_project_config() == ProjectConfig.defaults()
