.PHONY: install lint typecheck security test clean

install:
	pip install -e ".[dev]"

lint:
	ruff check src-python/

typecheck:
	mypy src-python/

security:
	bandit -r src-python/

test:
	pytest tests/ -v --tb=short

clean:
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null
	find . -type f -name '*.pyc' -delete

all: lint typecheck security test
