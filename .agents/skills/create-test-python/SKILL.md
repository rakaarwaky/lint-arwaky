---
name: create-test-python
description: "Generates contract, unit, integration, E2E, acceptance, and smoke test suites in tests/ (flat prefix naming), plus benchmark suites in benches/ (separate directory). Use when adding a new capability package, increasing coverage, preparing a release, or validating performance. Triggers: create tests python, add tests python, create test suite python, package tests python, e2e tests python, benchmark python."
metadata:
  tags: [python, testing, pytest, contract, unit, integration, e2e, acceptance, smoke, benchmark]
  related: [create-test-rust, create-test-typescript]
---
# Create Python Test Suite

## Directory Layout

Tests and benchmarks are **separated into distinct directories**:

```
packages/<name>/
├── src/
│   └── capabilities_my_class.py    # NO inline tests. Clean.
├── tests/                          # Contract, unit, integration, smoke, e2e, acceptance (flat prefix naming)
│   ├── contract_<package>.py
│   ├── unit_<package>_<module>.py
│   ├── integration_<package>.py
│   ├── smoke_<app>.py
│   ├── e2e_<flow>.py
│   └── acceptance_<FRD_ID>.py
├── benches/                        # Benchmark tests only
│   └── bench_<subject>.py
├── conftest.py                     # Shared fixtures (tests/ root)
└── pyproject.toml                  # pytest config + coverage targets
```

## Rules

- **Tests** (`tests/`): All contract, unit, integration, smoke, e2e, and acceptance tests live flat in `tests/` using filename prefixes as virtual subfolders
- **Benchmarks** (`benches/`): All benchmark tests live in a separate `benches/` directory
- No real subdirectories inside `tests/` — prefix IS the folder
- Prefix pattern: `<type>_<subject>.py`
- Contract tests verify class/interface implementation
- Unit tests cover happy path, edge cases, error paths
- Integration tests use the real DI container / entry point
- E2E tests hit the real API/CLI, assert on real output
- Acceptance tests map 1:1 to a business requirement FRD/PRD
- Smoke tests must complete in under 5 seconds
- Benchmark tests use `pytest-benchmark` or `timeit` — never hand-rolled timing
- Test IDs match the requirements section they validate

## Coverage Targets

| Layer        | Minimum |
| ------------ | ------- |
| Capabilities | 70%     |
| Agent        | 60%     |
| Utility      | 50%     |

## Naming Convention

```
tests/                          # Regular tests (fast, PR-gated)
├── contract_<package>.py
├── unit_<package>_<module>.py
├── integration_<package>.py
├── smoke_<app>.py
├── e2e_<flow>.py
└── acceptance_<FRD_ID>.py

benches/                        # Benchmark tests (slow, release-gated)
└── bench_<subject>.py
```

Example:

```
tests/
├── contract_aes.py
├── unit_aes_encrypt.py
├── unit_aes_decrypt.py
├── integration_aes.py
├── smoke_server.py
├── e2e_encrypt_decrypt_flow.py
├── acceptance_FRD_042.py
└── acceptance_FRD_043.py

benches/
└── bench_aes_throughput.py
```

## pyproject.toml for Benchmarks

Benchmarks live in `benches/`, configure pytest to exclude them from regular runs:

```toml
[tool.pytest.ini_options]
addopts = "--benchmark-disable"
markers = ["benchmark"]

[tool.coverage.run]
source = ["packages"]
```

Run benchmarks separately:

```bash
pytest benches/bench_aes_throughput.py --benchmark-only
```

## Test Types

| Prefix           | Directory  | Scope                    | Speed  | Runs when                |
| ---------------- | ---------- | ------------------------ | ------ | ------------------------ |
| `contract_`    | tests/     | Class/interface impl     | ms     | Every PR                 |
| `unit_`        | tests/     | One public function      | ms     | Every PR                 |
| `integration_` | tests/     | Package / DI wiring      | ms–s   | Every PR                 |
| `smoke_`       | tests/     | App boots + responds     | < 5s   | Every PR                 |
| `e2e_`         | tests/     | Full request lifecycle   | s      | Every PR (critical path) |
| `acceptance_`  | tests/     | Business requirement met | s      | Every PR / release gate  |
| `bench_`       | benches/   | Performance regression   | s–min | Release gate / nightly   |

## Workflow

```
Task Progress:
- [ ] Step 1: Analyze package / app structure
- [ ] Step 2: Identify untested public API
- [ ] Step 3: Write tests/contract_<package>.py
- [ ] Step 4: Write tests/unit_<package>_<module>.py
- [ ] Step 5: Write tests/integration_<package>.py
- [ ] Step 6: Write tests/smoke_<app>.py
- [ ] Step 7: Write tests/e2e_<flow>.py
- [ ] Step 8: Write tests/acceptance_<FRD_ID>.py
- [ ] Step 9: Write benches/bench_<subject>.py + register in pyproject.toml
- [ ] Step 10: Run suite, fix failures, repeat until green
- [ ] Step 11: Verify coverage + perf baseline
```

### Step 1: Analyze Package

```bash
ls packages/<name>/src/
ls packages/<name>/tests/ 2>/dev/null
pytest packages/<name>/ -v --collect-only | head -20
```

Identify layer, public classes, public functions, entry points.

### Step 2: Identify Gaps

For each public method: **"Does this function have a test?"**
No → add to test plan.

### Step 3: Contract Tests

Verify class/interface implementation exists and is callable:

```python
# tests/contract_aes.py
import pytest
from packages.src.capabilities_aes import AesCapability


class TestAesContract:
    """Contract tests for AES capability."""

    def test_implements_encrypt_interface(self):
        """Verify AesCapability has required methods."""
        cap = AesCapability()
        assert hasattr(cap, "encrypt")
        assert hasattr(cap, "decrypt")
        assert callable(getattr(cap, "encrypt"))
        assert callable(getattr(cap, "decrypt"))

    def test_implements_key_management(self):
        """Verify key management methods exist."""
        cap = AesCapability()
        assert hasattr(cap, "generate_key")
        assert hasattr(cap, "validate_key")
```

### Step 4: Unit Tests

One file per module or logical group. Public API only.

```python
# tests/unit_aes_encrypt.py
import pytest
from packages.src.capabilities_aes import AesCapability


class TestAesEncrypt:
    """Unit tests for AES encryption."""

    def setup_method(self):
        """Setup test fixtures."""
        self.key = b"0123456789abcdef" * 2  # 32 bytes
        self.cap = AesCapability(key=self.key)

    def test_encrypt_happy_path(self):
        """Test encryption returns valid ciphertext."""
        plaintext = b"hello world"
        result = self.cap.encrypt(plaintext)
        assert result is not None
        assert result != plaintext  # Must be different from plaintext

    def test_encrypt_empty_input(self):
        """Test encryption with empty input."""
        result = self.cap.encrypt(b"")
        assert result is not None
        assert len(result) > 0  # Should produce some output

    def test_encrypt_invalid_key_length(self):
        """Test encryption with invalid key length raises error."""
        short_key = b"short"
        with pytest.raises(ValueError, match="Invalid key length"):
            AesCapability(key=short_key)

    def test_encrypt_unicode_characters(self):
        """Test encryption handles unicode characters."""
        plaintext = "café résumé Ñ".encode("utf-8")
        result = self.cap.encrypt(plaintext)
        assert result is not None
        # Decrypt should restore original
        decrypted = self.cap.decrypt(result)
        assert decrypted == plaintext

    def test_encrypt_binary_data(self):
        """Test encryption handles binary data."""
        import os
        plaintext = os.urandom(1024)
        result = self.cap.encrypt(plaintext)
        assert result is not None
        decrypted = self.cap.decrypt(result)
        assert decrypted == plaintext
```

### Step 5: Integration Tests

Test cross-capability interaction using real DI container:

```python
# tests/integration_aes.py
import pytest
from packages.src.root_container import build_container


class TestAesIntegration:
    """Integration tests for AES capability wiring."""

    def test_di_container_wires_aes_capability(self):
        """Verify DI container wires AES capability correctly."""
        container = build_container()
        cap = container.get("aes_capability")
        assert cap is not None
        assert hasattr(cap, "encrypt")

    def test_encrypt_then_decrypt_roundtrip(self):
        """Test encrypt → decrypt roundtrip through container."""
        container = build_container()
        enc = container.get("aes_capability").encrypt
        dec = container.get("aes_capability").decrypt

        plaintext = b"secret message"
        ciphertext = enc(plaintext)
        result = dec(ciphertext)
        assert result == plaintext

    def test_multiple_encryptions_produce_different_ciphertext(self):
        """Verify each encryption produces unique ciphertext (IV/nonce)."""
        container = build_container()
        enc = container.get("aes_capability").encrypt

        ct1 = enc(b"same message")
        ct2 = enc(b"same message")
        assert ct1 != ct2  # Should differ due to random IV
```

### Step 6: Smoke Test

One file. One test. Boots the app. Fast.

```python
# tests/smoke_server.py
import pytest
import requests


@pytest.mark.smoke
def test_app_boots_and_responds():
    """Smoke test: app boots and responds to health check."""
    resp = requests.get("http://localhost:8000/health", timeout=5)
    assert resp.status_code == 200
    assert resp.json()["status"] == "ok"
```

If this fails, nothing else matters.

### Step 7: E2E Tests

Full request → all layers → real output. No internal mocks.

```python
# tests/e2e_encrypt_decrypt_flow.py
import pytest
import requests


@pytest.mark.e2e
def test_full_encrypt_decrypt_lifecycle():
    """End-to-end: encrypt via API, decrypt via API."""
    # Encrypt
    resp = requests.post(
        "http://localhost:8000/api/encrypt",
        json={"plaintext": "hello world"},
    )
    assert resp.status_code == 200
    body = resp.json()
    assert "ciphertext" in body
    assert len(body["ciphertext"]) > 0

    # Decrypt
    resp2 = requests.post(
        "http://localhost:8000/api/decrypt",
        json={"ciphertext": body["ciphertext"]},
    )
    assert resp2.status_code == 200
    body2 = resp2.json()
    assert body2["plaintext"] == "hello world"
```

### Step 8: Acceptance Tests

One file per requirement. Filename contains the REQ ID.

```python
# tests/acceptance_FRD_042.py
"""FRD-042: User can encrypt a payload with AES-256-GCM."""
import pytest
import requests


@pytest.mark.acceptance
@pytest.mark.frq("FRD-042")
def test_frq_042_encrypt_payload_aes256gcm():
    """FRQ-042: Encrypt payload using AES-256-GCM algorithm."""
    resp = requests.post(
        "http://localhost:8000/api/encrypt",
        json={
            "plaintext": "sensitive data",
            "algorithm": "aes-256-gcm",
        },
    )
    assert resp.status_code == 200
    body = resp.json()
    assert body["algorithm"] == "aes-256-gcm"
    assert body["ciphertext"] != "sensitive data"  # Must be encrypted


@pytest.mark.acceptance
@pytest.mark.frq("FRD-043")
def test_frq_043_wrong_key_rejected():
    """FRQ-043: Decryption with wrong key returns 401."""
    resp = requests.post(
        "http://localhost:8000/api/decrypt",
        json={
            "ciphertext": "valid_ct",
            "key": "wrong_key",
        },
    )
    assert resp.status_code == 401
```

### Step 9: Benchmark Tests

```python
# benches/bench_aes_throughput.py
import pytest
from packages.src.capabilities_aes import AesCapability


@pytest.fixture
def aes_capability():
    """Provide AES capability for benchmarks."""
    key = b"0123456789abcdef" * 2  # 32 bytes
    return AesCapability(key=key)


def bench_encrypt_small(aes_capability, benchmark):
    """Benchmark encryption of small payload (64 bytes)."""
    payload = b"0" * 64
    benchmark(aes_capability.encrypt, payload)


def bench_encrypt_medium(aes_capability, benchmark):
    """Benchmark encryption of medium payload (1KB)."""
    payload = b"0" * 1024
    benchmark(aes_capability.encrypt, payload)


def bench_encrypt_large(aes_capability, benchmark):
    """Benchmark encryption of large payload (64KB)."""
    payload = b"0" * 65536
    benchmark(aes_capability.encrypt, payload)
```

Run benchmarks:

```bash
pytest benches/bench_aes_throughput.py --benchmark-only --benchmark-columns=mean,stdmin,stdmax
```

### Step 10: Run and Fix

```bash
# Run all tests (excludes benches/)
pytest packages/<name>/tests/ -v

# Run with coverage
pytest packages/<name>/tests/ --cov=packages/<name> --cov-report=html

# Fix failures → re-run → repeat until green
```

### Step 11: Verify Coverage + Perf

```bash
# Check coverage (tests/ only)
pytest packages/<name>/tests/ --cov=packages/<name> --cov-report=term-missing --cov-fail-under=<target>

# Run benchmarks (benches/)
pytest benches/bench_<subject>.py --benchmark-only --benchmark-compare=<last-release-tag>
```

Coverage below threshold → return to Step 2.
Perf regressed > 10% → investigate before merging.

## Quick Reference

```bash
# Run all tests (tests/ only, excludes benches/)
pytest packages/<name>/tests/ -v

# Run one contract test file
pytest tests/contract_aes.py -v

# Run one unit test file
pytest tests/unit_aes_encrypt.py -v

# Run smoke tests only
pytest tests/smoke_server.py -m smoke -v

# Run E2E tests only
pytest tests/e2e_encrypt_decrypt_flow.py -m e2e -v

# Run benchmarks (benches/ only)
pytest benches/bench_aes_throughput.py --benchmark-only

# Check coverage
pytest packages/<name>/tests/ --cov=packages/<name> --cov-report=term-missing

# Run with stdout
pytest packages/<name>/tests/ -v --capture=no
```

## Directory Layout

```
packages/<name>/
├── src/
│   └── capabilities_my_class.py    # NO inline tests. Clean.
├── tests/                          # Regular tests (PR-gated, fast)
│   ├── contract_aes.py
│   ├── unit_aes_encrypt.py
│   ├── unit_aes_decrypt.py
│   ├── integration_aes.py
│   ├── smoke_server.py
│   ├── e2e_encrypt_decrypt_flow.py
│   ├── acceptance_FRD_042.py
│   └── acceptance_FRD_043.py
├── benches/                        # Benchmark tests (release-gated, slow)
│   └── bench_aes_throughput.py
├── conftest.py                     # Shared fixtures (tests/ root)
└── pyproject.toml                  # pytest config + coverage targets
```

## Common Mistakes (AVOID)

- ❌ **Testing implementation details**: Test public APIs, not internal methods (`_private`)
- ❌ **Missing edge cases**: Always test error paths, invalid inputs, empty values
- ❌ **Over-mocking**: Only mock external dependencies (DB, HTTP), not internal logic
- ❌ **Skipping fixture setup**: Use `setup_method` / `fixture` for consistent test state
- ❌ **Hardcoding assertions**: Use parameterized tests for repetitive scenarios
- ❌ **Running benchmarks by default**: Benchmarks should be opt-in (`--benchmark-only`)

## pytest Markers Reference

```python
# Register custom markers in pyproject.toml
[tool.pytest.ini_options]
markers = [
    "smoke: smoke tests (fast, app boots)",
    "e2e: end-to-end tests (full lifecycle)",
    "acceptance: acceptance tests (requirement validation)",
    "benchmark: performance benchmarks (slow)",
]
```

Run subsets:

```bash
pytest -m smoke        # smoke only
pytest -m e2e          # e2e only
pytest -m benchmark    # benchmark only
pytest -m "not benchmark"  # everything except benchmarks
```
