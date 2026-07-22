---
name: create-test-rust
description: "Generates contract, unit, integration, E2E, acceptance, smoke, and benchmark test suites for Rust crates and applications. All tests live flat in tests/ using filename prefixes as virtual subfolders. Use when adding a new capability crate, increasing coverage, preparing a release, or validating performance. Triggers: create tests rust, add tests rust, create test suite rust, crate tests rust, e2e tests rust, benchmark rust."
metadata:
  tags: [rust, testing, contract, unit, integration, e2e, acceptance, smoke, benchmark]
  related: [create-test-python, create-test-typescript]
---
# Create Rust Test Suite

## Rules

- ALL tests live in `tests/`
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

tests/
├── contract_<crate>.rs
├── unit_<crate>_<module>.rs
├── integration_<crate>.rs
├── smoke_<app>.rs
├── e2e_<flow>.rs
├── acceptance_<FR_id>.rs
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
├── acceptance_FRD_043.rs
└── bench_aes_throughput.rs

## Cargo.toml for Benchmarks

Since benches live in `tests/`, declare the path explicitly:

```toml
[[bench]]
name = "bench_aes_throughput"
path = "tests/bench_aes_throughput.rs"
harness = false
```

## Test Types

| Prefix           | Scope                    | Speed  | Runs when                |
| ---------------- | ------------------------ | ------ | ------------------------ |
| `contract_`    | Trait impl exists        | ms     | Every PR                 |
| `unit_`        | One public function      | ms     | Every PR                 |
| `integration_` | Crate / DI wiring        | ms–s  | Every PR                 |
| `smoke_`       | App boots + responds     | < 5s   | Every PR                 |
| `e2e_`         | Full request lifecycle   | s      | Every PR (critical path) |
| `acceptance_`  | Business requirement met | s      | Every PR / release gate  |
| `bench_`       | Performance regression   | s–min | Release gate / nightly   |

## Workflow

```
Task Progress:
- [ ] Step 1: Analyze crate / app structure
- [ ] Step 2: Identify untested public API
- [ ] Step 3: Write contract_<crate>.rs
- [ ] Step 4: Write unit_<crate>_<module>.rs
- [ ] Step 5: Write integration_<crate>.rs
- [ ] Step 6: Write smoke_<app>.rs
- [ ] Step 7: Write e2e_<flow>.rs
- [ ] Step 8: Write acceptance_<FR_id>.rs
- [ ] Step 9: Write bench_<subject>.rs + register in Cargo.toml
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
// tests/bench_aes_throughput.rs
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
path = "tests/bench_aes_throughput.rs"
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
cargo test -p <name>                                    # all tests
cargo test -p <name> --test contract_aes                # one file
cargo test -p <name> --test unit_aes_encrypt            # one unit file
cargo test -p <name> --test smoke_server                # smoke only
cargo test -p <name> --test e2e_encrypt_decrypt_flow    # one e2e
cargo test -p <name> --test acceptance_req_042          # one acceptance
cargo bench -p <name>                                   # benchmarks
cargo tarpaulin -p <name>                               # coverage
cargo test -p <name> -- --nocapture                     # with stdout
```

## Directory Layout

```
crates/<name>/
├── src/
│   └── lib.rs                      # NO inline tests. Clean.
├── tests/
│   ├── contract_aes.rs
│   ├── unit_aes_encrypt.rs
│   ├── unit_aes_decrypt.rs
│   ├── integration_aes.rs
│   ├── smoke_server.rs
│   ├── e2e_encrypt_decrypt_flow.rs
│   ├── acceptance_req_042.rs
│   ├── acceptance_req_043.rs
│   └── bench_aes_throughput.rs
└── Cargo.toml                      # [[bench]] path → tests/bench_*.rs
```

```
```
