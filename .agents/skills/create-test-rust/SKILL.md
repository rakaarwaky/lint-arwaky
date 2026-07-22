---
name: create-test-rust
description: "Generates contract, unit, integration, E2E, acceptance, and smoke test suites in tests/ (flat prefix naming), plus benchmark suites in benches/ (separate directory). Use when adding a new capability crate, increasing coverage, preparing a release, or validating performance. Triggers: create tests rust, add tests rust, create test suite rust, crate tests rust, e2e tests rust, benchmark rust."
metadata:
  tags: [rust, testing, contract, unit, integration, e2e, acceptance, smoke, benchmark]
  related: [create-test-python, create-test-typescript]
---
# Create Rust Test Suite

## Directory Layout

Tests and benchmarks are **separated into distinct directories**:

```
crates/<name>/
├── src/
│   └── lib.rs                      # NO inline tests. Clean.
├── tests/                          # Contract, unit, integration, smoke, e2e, acceptance
│   ├── contract_<crate>.rs
│   ├── unit_<crate>_<module>.rs
│   ├── integration_<crate>.rs
│   ├── smoke_<app>.rs
│   ├── e2e_<flow>.rs
│   └── acceptance_<FR_id>.rs
├── benches/                        # Benchmark tests only
│   └── bench_<subject>.rs
└── Cargo.toml                      # [[bench]] path → benches/bench_*.rs
```

## Rules

- **Tests** (`tests/`): All contract, unit, integration, smoke, e2e, and acceptance tests live flat in `tests/` using filename prefixes as virtual subfolders
- **Benchmarks** (`benches/`): All benchmark tests live in a separate `benches/` directory
- No real subdirectories inside `tests/` — prefix IS the folder
- Prefix pattern: `<type>_<subject>.rs`
- Contract tests verify trait implementation
- Unit tests cover happy path, edge cases, error paths
- Integration tests use the real DI container
- E2E tests hit the real entry point, assert on real output
- Acceptance tests map 1:1 to a business requirement FRD/PRD
- Smoke tests must complete in under 5 seconds
- Benchmark tests use `criterion` — never hand-rolled timing
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
├── contract_<crate>.rs
├── unit_<crate>_<module>.rs
├── integration_<crate>.rs
├── smoke_<app>.rs
├── e2e_<flow>.rs
└── acceptance_<FR_id>.rs

benches/                        # Benchmark tests (slow, release-gated)
└── bench_<subject>.rs
```

Example:

```
tests/
├── contract_aes.rs
├── unit_aes_encrypt.rs
├── unit_aes_decrypt.rs
├── integration_aes.rs
├── smoke_server.rs
├── e2e_encrypt_decrypt_flow.rs
├── acceptance_FRD_042.rs
└── acceptance_FRD_043.rs

benches/
└── bench_aes_throughput.rs
```

## Cargo.toml for Benchmarks

Benchmarks live in `benches/`, declare the path explicitly:

```toml
[[bench]]
name = "bench_aes_throughput"
path = "benches/bench_aes_throughput.rs"
harness = false
```

## Test Types

| Prefix           | Directory  | Scope                    | Speed  | Runs when                |
| ---------------- | ---------- | ------------------------ | ------ | ------------------------ |
| `contract_`    | tests/     | Trait impl exists        | ms     | Every PR                 |
| `unit_`        | tests/     | One public function      | ms     | Every PR                 |
| `integration_` | tests/     | Crate / DI wiring        | ms–s   | Every PR                 |
| `smoke_`       | tests/     | App boots + responds     | < 5s   | Every PR                 |
| `e2e_`         | tests/     | Full request lifecycle   | s      | Every PR (critical path) |
| `acceptance_`  | tests/     | Business requirement met | s      | Every PR / release gate  |
| `bench_`       | benches/   | Performance regression   | s–min | Release gate / nightly   |

## Workflow

```
Task Progress:
- [ ] Step 1: Analyze crate / app structure
- [ ] Step 2: Identify untested public API
- [ ] Step 3: Write tests/contract_<crate>.rs
- [ ] Step 4: Write tests/unit_<crate>_<module>.rs
- [ ] Step 5: Write tests/integration_<crate>.rs
- [ ] Step 6: Write tests/smoke_<app>.rs
- [ ] Step 7: Write tests/e2e_<flow>.rs
- [ ] Step 8: Write tests/acceptance_<FR_id>.rs
- [ ] Step 9: Write benches/bench_<subject>.rs + register in Cargo.toml
- [ ] Step 10: Run suite, fix failures, repeat until green
- [ ] Step 11: Verify coverage + perf baseline
```

### Step 1: Analyze Crate

```bash
ls crates/<name>/src/
ls crates/<name>/tests/ 2>/dev/null
cargo test -p <name> 2>&1 | head -20
```

Identify layer, public traits, public structs, entry points.

### Step 2: Identify Gaps

For each public method: **"Does this function have a test?"**
No → add to test plan.

### Step 3: Contract Tests

```rust
// tests/contract_aes.rs
use aes_crate::*;

#[test]
fn implements_encrypt_trait() {
    fn assert_trait<T: Encrypt>() {}
    assert_trait::<AesCapability>();
}

#[test]
fn implements_decrypt_trait() {
    fn assert_trait<T: Decrypt>() {}
    assert_trait::<AesCapability>();
}
```

### Step 4: Unit Tests

One file per module or logical group. Public API only.

```rust
// tests/unit_aes_encrypt.rs
use aes_crate::*;

#[test]
fn encrypt_happy_path() {
    let sut = AesCapability::new(test_key());
    let result = sut.encrypt(b"hello world");
    assert!(result.is_ok());
}

#[test]
fn encrypt_empty_input() {
    let sut = AesCapability::new(test_key());
    let result = sut.encrypt(b"");
    assert!(result.is_err());
}

#[test]
fn encrypt_invalid_key_length() {
    let sut = AesCapability::new(b"short");
    let result = sut.encrypt(b"data");
    assert_eq!(result.unwrap_err(), AesError::InvalidKeyLength);
}
```

### Step 5: Integration Tests

```rust
// tests/integration_aes.rs
use aes_crate::*;

#[test]
fn di_container_wires_aes_capability() {
    let container = build_container();
    let cap = container.resolve::<AesCapability>();
    assert!(cap.is_some());
}

#[test]
fn encrypt_then_decrypt_roundtrip() {
    let container = build_container();
    let enc = container.resolve::<AesCapability>().unwrap();
    let dec = container.resolve::<AesCapability>().unwrap();

    let ct = enc.encrypt(b"secret").unwrap();
    let pt = dec.decrypt(&ct).unwrap();
    assert_eq!(pt, b"secret");
}
```

### Step 6: Smoke Test

One file. One test. Boots the app. Fast.

```rust
// tests/smoke_server.rs
#[tokio::test]
async fn app_boots_and_responds() {
    let app = spawn_app().await;
    let resp = app.client.get("/health").send().await;
    assert_eq!(resp.status(), 200);
}
```

If this fails, nothing else matters.

### Step 7: E2E Tests

Full request → all layers → real output. No internal mocks.

```rust
// tests/e2e_encrypt_decrypt_flow.rs
#[tokio::test]
async fn full_encrypt_decrypt_lifecycle() {
    let app = spawn_app().await;

    let resp = app.client
        .post("/api/encrypt")
        .json(&serde_json::json!({ "plaintext": "hello" }))
        .send()
        .await;

    assert_eq!(resp.status(), 200);
    let body: EncryptResponse = resp.json().await;
    assert!(!body.ciphertext.is_empty());

    let resp2 = app.client
        .post("/api/decrypt")
        .json(&serde_json::json!({ "ciphertext": body.ciphertext }))
        .send()
        .await;

    let body2: DecryptResponse = resp2.json().await;
    assert_eq!(body2.plaintext, "hello");
}
```

### Step 8: Acceptance Tests

One file per requirement. Filename contains the REQ ID.

```rust
// tests/acceptance_req_042.rs
/// REQ-042: User can encrypt a payload with AES-256-GCM
#[tokio::test]
async fn req_042_encrypt_payload_aes256gcm() {
    let app = spawn_app().await;

    let resp = app.client
        .post("/api/encrypt")
        .json(&serde_json::json!({
            "plaintext": "sensitive data",
            "algorithm": "aes-256-gcm"
        }))
        .send()
        .await;

    assert_eq!(resp.status(), 200);
    let body: EncryptResponse = resp.json().await;
    assert_eq!(body.algorithm, "aes-256-gcm");
    assert_ne!(body.ciphertext, "sensitive data");
}
```

```rust
// tests/acceptance_req_043.rs
/// REQ-043: Decryption with wrong key returns 401
#[tokio::test]
async fn req_043_wrong_key_rejected() {
    let app = spawn_app().await;

    let resp = app.client
        .post("/api/decrypt")
        .json(&serde_json::json!({
            "ciphertext": "valid_ct",
            "key": "wrong_key"
        }))
        .send()
        .await;

    assert_eq!(resp.status(), 401);
}
```

### Step 9: Benchmark Tests

```rust
// benches/bench_aes_throughput.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use aes_crate::AesCapability;

fn bench_encrypt(c: &mut Criterion) {
    let cap = AesCapability::new(&[0u8; 32]);
    let mut group = c.benchmark_group("encrypt");

    for size in [64, 1024, 65536] {
        let payload = vec![0u8; size];
        group.bench_with_input(
            BenchmarkId::new("aes256gcm", size),
            &payload,
            |b, data| b.iter(|| cap.encrypt(data)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_encrypt);
criterion_main!(benches);
```

Register in `Cargo.toml` with explicit path:

```toml
[[bench]]
name = "bench_aes_throughput"
path = "benches/bench_aes_throughput.rs"
harness = false
```

### Step 10: Run and Fix

```bash
cargo test -p <name> -- --nocapture
# Fix failures → re-run → repeat until green
```

### Step 11: Verify Coverage + Perf

```bash
cargo tarpaulin -p <name> --fail-under <target>
cargo bench -p <name> -- --baseline <last-release>
```

Coverage below threshold → return to Step 2.
Perf regressed > 10% → investigate before merging.

## Quick Reference

```bash
cargo test -p <name>                                    # all tests (tests/ only)
cargo test -p <name> --test contract_aes                # one file
cargo test -p <name> --test unit_aes_encrypt            # one unit file
cargo test -p <name> --test smoke_server                # smoke only
cargo test -p <name> --test e2e_encrypt_decrypt_flow    # one e2e
cargo test -p <name> --test acceptance_req_042          # one acceptance
cargo bench -p <name>                                   # benchmarks (benches/ only)
cargo tarpaulin -p <name>                               # coverage
cargo test -p <name> -- --nocapture                     # with stdout
```

## Directory Layout

```
crates/<name>/
├── src/
│   └── lib.rs                      # NO inline tests. Clean.
├── tests/                          # Regular tests (PR-gated, fast)
│   ├── contract_aes.rs
│   ├── unit_aes_encrypt.rs
│   ├── unit_aes_decrypt.rs
│   ├── integration_aes.rs
│   ├── smoke_server.rs
│   ├── e2e_encrypt_decrypt_flow.rs
│   ├── acceptance_req_042.rs
│   └── acceptance_req_043.rs
├── benches/                        # Benchmark tests (release-gated, slow)
│   └── bench_aes_throughput.rs
└── Cargo.toml                      # [[bench]] path → benches/bench_*.rs
```
