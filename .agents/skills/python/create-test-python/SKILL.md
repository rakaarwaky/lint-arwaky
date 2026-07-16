---
name: create-test-python
description: "Create comprehensive test suites for Python modules following pytest conventions and project testing standards."
version: 1.0.0
category: testing
tags:
  [
    python,
    testing,
    pytest,
    unittest,
    mocking,
    fixtures,
  ]
triggers:
  - "create test python"
  - "add tests python"
  - "write unit tests python"
dependencies: []
related:
  - add-docs-python
  - cleanup-files-python
---

# create-test-python

## Purpose

Create comprehensive test suites for Python modules following pytest conventions and project testing standards. Ensures all public APIs are tested with proper fixtures and mocking.

## Rules

### Test Structure

- Use `pytest` framework (not unittest)
- Name test files: `test_<module>.py`
- Name test functions: `test_<function>_<scenario>.py`
- Place tests in same directory structure as source

### Testing Conventions

- Use `assert` statements (not `self.assertEqual`)
- Use fixtures for setup/teardown
- Mock external dependencies with `unittest.mock`
- Test edge cases, not just happy paths

## When to Use

- After implementing new functionality
- Before committing changes
- When adding public APIs that need testing

## The Fundamental Question

> **"Is this functionality tested?"**

If no → **Create test suite**

## Workflow

### Step 1: Identify Testable Components

List all public classes, functions, and methods to test:

```bash
# Find public APIs
grep -rn "^class \|^def " modules/*/src/ | grep -v "^_"
```

### Step 2: Create Test File

Create `test_<module>.py` with test structure:

```python
import pytest
from unittest.mock import Mock, patch
from modules.src.capabilities_my_class import MyClass


class TestMyClass:
    def setup_method(self):
        """Setup test fixtures."""
        self.mock_dep = Mock()
        self.test_obj = MyClass(self.mock_dep)

    def test_method_returns_expected(self):
        """Test method returns expected value."""
        result = self.test_obj.method()
        assert result == expected_value

    def test_method_raises_on_invalid_input(self):
        """Test method raises on invalid input."""
        with pytest.raises(ValueError):
            self.test_obj.method(invalid_input)
```

### Step 3: Add Fixtures

Create conftest.py for shared fixtures:

```python
# conftest.py
import pytest

@pytest.fixture
def mock_filesystem():
    """Mock filesystem operations."""
    with patch('os.path') as mock:
        yield mock
```

### Step 4: Mock External Dependencies

Use `unittest.mock` to isolate tests:

```python
from unittest.mock import patch, MagicMock

@patch('modules.src.infrastructure_db.connect')
def test_database_operation(mock_connect):
    """Test database operation with mocked connection."""
    mock_connect.return_value = MagicMock()
    # ... test logic
```

### Step 5: Run Tests

```bash
# Run all tests
pytest modules/ -v

# Run with coverage
pytest modules/ --cov=modules --cov-report=html
```

## Verification Checklist

- [ ] All public APIs have test coverage
- [ ] Test files follow naming conventions (`test_*.py`)
- [ ] Fixtures used for setup/teardown
- [ ] External dependencies mocked appropriately
- [ ] Edge cases and error paths tested
- [ ] Tests run successfully with pytest

## Quick Commands

```bash
# Run all tests
pytest modules/ -v

# Run with coverage report
pytest modules/ --cov=modules --cov-report=html

# Find untested files
find modules/*/src/ -name "*.py" ! -path "*test*" | while read f; do
    base=$(basename "$f" .py)
    grep -rl "^from $base import\|^import $base" modules/*/tests/ || echo "UNTESTED: $base"
done

# Check test coverage
pytest modules/ --cov-report=term-missing
```

## Common Mistakes (AVOID)

- ❌ **Testing implementation details**: Test public APIs, not internal methods
- ❌ **Missing edge cases**: Always test error paths and invalid inputs
- ❌ **Over-mocking**: Only mock external dependencies, not internal logic
- ❌ **Skipping fixture setup**: Use fixtures for consistent test state
