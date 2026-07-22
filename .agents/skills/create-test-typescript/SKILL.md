---
name: create-test-typescript
description: "Generates contract, unit, integration, E2E, acceptance, smoke, and benchmark test suites for TypeScript packages and applications. All tests live flat in tests/ using filename prefixes as virtual subfolders. Use when adding a new capability package, increasing coverage, preparing a release, or validating performance. Triggers: create tests typescript, add tests typescript, create test suite typescript, package tests typescript, e2e tests typescript, benchmark typescript."
metadata:
  tags: [typescript, testing, vitest, jest, contract, unit, integration, e2e, acceptance, smoke, benchmark]
  related: [create-test-rust, create-test-python]
---
# Create TypeScript Test Suite

## Rules

- ALL tests live in `tests/`
- No real subdirectories inside `tests/` — prefix IS the folder
- Prefix pattern: `<type>_<subject>.ts`
- Contract tests verify class/interface implementation
- Unit tests cover happy path, edge cases, error paths
- Integration tests use the real DI container / entry point
- E2E tests hit the real API/CLI, assert on real output
- Acceptance tests map 1:1 to a business requirement FRD/PRD
- Smoke tests must complete in under 5 seconds
- Benchmark tests use `vitest/benchmark` or `benchmark.js` — never hand-rolled timing
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
├── contract_<package>.ts
├── unit_<package>_<module>.ts
├── integration_<package>.ts
├── smoke_<app>.ts
├── e2e_<flow>.ts
├── acceptance_<FRD_ID>.ts
└── bench_<subject>.ts
```

Example:

```
tests/
├── contract_aes.ts
├── unit_aes_encrypt.ts
├── unit_aes_decrypt.ts
├── integration_aes.ts
├── smoke_server.ts
├── e2e_encrypt_decrypt_flow.ts
├── acceptance_FRD_042.ts
├── acceptance_FRD_043.ts
└── bench_aes_throughput.ts
```

## vitest.config.ts for Benchmarks

Since benches live in `tests/`, configure benchmark mode:

```typescript
// vitest.config.ts
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    globals: true,
    environment: "node",
    include: ["tests/**/*.ts"],
    exclude: ["tests/bench_*.ts"], // benchmarks run separately
  },
});
```

Run benchmarks:

```bash
vitest bench tests/bench_aes_throughput.ts
```

## Test Types

| Prefix           | Scope                    | Speed  | Runs when                |
| ---------------- | ------------------------ | ------ | ------------------------ |
| `contract_`    | Class/interface impl     | ms     | Every PR                 |
| `unit_`        | One public function      | ms     | Every PR                 |
| `integration_` | Package / DI wiring      | ms–s   | Every PR                 |
| `smoke_`       | App boots + responds     | < 5s   | Every PR                 |
| `e2e_`         | Full request lifecycle   | s      | Every PR (critical path) |
| `acceptance_`  | Business requirement met | s      | Every PR / release gate  |
| `bench_`       | Performance regression   | s–min | Release gate / nightly   |

## Workflow

```
Task Progress:
- [ ] Step 1: Analyze package / app structure
- [ ] Step 2: Identify untested public API
- [ ] Step 3: Write contract_<package>.ts
- [ ] Step 4: Write unit_<package>_<module>.ts
- [ ] Step 5: Write integration_<package>.ts
- [ ] Step 6: Write smoke_<app>.ts
- [ ] Step 7: Write e2e_<flow>.ts
- [ ] Step 8: Write acceptance_<FRD_ID>.ts
- [ ] Step 9: Write bench_<subject>.ts + register in vitest.config.ts
- [ ] Step 10: Run suite, fix failures, repeat until green
- [ ] Step 11: Verify coverage + perf baseline
```

### Step 1: Analyze Package

```bash
ls packages/<name>/src/
ls packages/<name>/tests/ 2>/dev/null
npx vitest --dir packages/<name>/ -v --collect-only | head -20
```

Identify layer, public classes, public functions, entry points.

### Step 2: Identify Gaps

For each public method: **"Does this function have a test?"**
No → add to test plan.

### Step 3: Contract Tests

Verify class/interface implementation exists and is callable:

```typescript
// tests/contract_aes.ts
import { describe, it, expect } from "vitest";
import { AesCapability } from "../src/capabilities_aes";

describe("AesContract", () => {
  it("implements encrypt method", () => {
    const cap = new AesCapability();
    expect(typeof cap.encrypt).toBe("function");
    expect(cap.decrypt).toBeDefined();
  });

  it("implements key management methods", () => {
    const cap = new AesCapability();
    expect(cap.generateKey).toBeDefined();
    expect(cap.validateKey).toBeDefined();
  });
});
```

### Step 4: Unit Tests

One file per module or logical group. Public API only.

```typescript
// tests/unit_aes_encrypt.ts
import { describe, it, expect, beforeEach } from "vitest";
import { AesCapability } from "../src/capabilities_aes";

describe("AesEncrypt", () => {
  let cap: AesCapability;
  const key = Buffer.from("0123456789abcdef" * 2, "hex").slice(0, 32);

  beforeEach(() => {
    cap = new AesCapability(key);
  });

  it("encrypts plaintext to ciphertext", () => {
    const plaintext = Buffer.from("hello world");
    const result = cap.encrypt(plaintext);
    expect(result).toBeDefined();
    expect(result).not.toEqual(plaintext); // Must be different
  });

  it("handles empty input", () => {
    const result = cap.encrypt(Buffer.from(""));
    expect(result).toBeDefined();
    expect(result!.length).toBeGreaterThan(0);
  });

  it("throws on invalid key length", () => {
    expect(() => new AesCapability(Buffer.from("short"))).toThrow(
      "Invalid key length"
    );
  });

  it("encrypts unicode characters", () => {
    const plaintext = Buffer.from("café résumé Ñ");
    const result = cap.encrypt(plaintext);
    expect(result).toBeDefined();
    const decrypted = cap.decrypt(result!);
    expect(decrypted).toEqual(plaintext);
  });

  it("encrypts binary data", () => {
    const plaintext = Buffer.from([0, 1, 2, 3, 4, 5]);
    const result = cap.encrypt(plaintext);
    expect(result).toBeDefined();
    const decrypted = cap.decrypt(result!);
    expect(decrypted).toEqual(plaintext);
  });
});
```

### Step 5: Integration Tests

Test cross-capability interaction using real DI container:

```typescript
// tests/integration_aes.ts
import { describe, it, expect } from "vitest";
import { buildContainer } from "../src/root_container";

describe("AesIntegration", () => {
  it("DI container wires AES capability", () => {
    const container = buildContainer();
    const cap = container.get("aes_capability");
    expect(cap).toBeDefined();
    expect(typeof cap.encrypt).toBe("function");
  });

  it("encrypts then decrypts roundtrip", () => {
    const container = buildContainer();
    const enc = container.get("aes_capability").encrypt;
    const dec = container.get("aes_capability").decrypt;

    const plaintext = Buffer.from("secret message");
    const ciphertext = enc(plaintext);
    const result = dec(ciphertext!);
    expect(result).toEqual(plaintext);
  });

  it("multiple encryptions produce different ciphertext", () => {
    const container = buildContainer();
    const enc = container.get("aes_capability").encrypt;

    const ct1 = enc(Buffer.from("same message"));
    const ct2 = enc(Buffer.from("same message"));
    expect(ct1).not.toEqual(ct2); // Should differ due to random IV
  });
});
```

### Step 6: Smoke Test

One file. One test. Boots the app. Fast.

```typescript
// tests/smoke_server.ts
import { describe, it, expect } from "vitest";
import request from "supertest";
import { createApp } from "../src/app";

describe("Smoke", () => {
  it("app boots and responds to health check", async () => {
    const app = createApp();
    const resp = await request(app).get("/health").expect(200);
    expect(resp.body.status).toBe("ok");
  });
});
```

If this fails, nothing else matters.

### Step 7: E2E Tests

Full request → all layers → real output. No internal mocks.

```typescript
// tests/e2e_encrypt_decrypt_flow.ts
import { describe, it, expect } from "vitest";
import request from "supertest";
import { createApp } from "../src/app";

describe("E2E Encrypt Decrypt Flow", () => {
  it("full encrypt decrypt lifecycle", async () => {
    const app = createApp();

    // Encrypt
    const resp = await request(app)
      .post("/api/encrypt")
      .send({ plaintext: "hello world" })
      .expect(200);

    expect(resp.body.ciphertext).toBeDefined();
    expect(resp.body.ciphertext.length).toBeGreaterThan(0);

    // Decrypt
    const resp2 = await request(app)
      .post("/api/decrypt")
      .send({ ciphertext: resp.body.ciphertext })
      .expect(200);

    expect(resp2.body.plaintext).toBe("hello world");
  });
});
```

### Step 8: Acceptance Tests

One file per requirement. Filename contains the REQ ID.

```typescript
// tests/acceptance_FRD_042.ts
/**
 * FRD-042: User can encrypt a payload with AES-256-GCM
 */
import { describe, it, expect } from "vitest";
import request from "supertest";
import { createApp } from "../src/app";

describe("FRD-042", () => {
  it("encrypts payload using AES-256-GCM algorithm", async () => {
    const app = createApp();
    const resp = await request(app)
      .post("/api/encrypt")
      .send({ plaintext: "sensitive data", algorithm: "aes-256-gcm" })
      .expect(200);

    expect(resp.body.algorithm).toBe("aes-256-gcm");
    expect(resp.body.ciphertext).not.toBe("sensitive data"); // Must be encrypted
  });
});

/**
 * FRD-043: Decryption with wrong key returns 401
 */
describe("FRD-043", () => {
  it("rejects decryption with wrong key", async () => {
    const app = createApp();
    const resp = await request(app)
      .post("/api/decrypt")
      .send({ ciphertext: "valid_ct", key: "wrong_key" })
      .expect(401);

    expect(resp.body.error).toBeDefined();
  });
});
```

### Step 9: Benchmark Tests

```typescript
// tests/bench_aes_throughput.ts
import { bench, describe, fit } from "vitest/plugin/testing";
import { AesCapability } from "../src/capabilities_aes";

const key = Buffer.from("0123456789abcdef" * 2, "hex").slice(0, 32);

describe("AES Benchmarks", () => {
  bench("encrypt small (64 bytes)", () => {
    const cap = new AesCapability(key);
    const payload = Buffer.from("0".repeat(64));
    cap.encrypt(payload);
  });

  bench("encrypt medium (1KB)", () => {
    const cap = new AesCapability(key);
    const payload = Buffer.from("0".repeat(1024));
    cap.encrypt(payload);
  });

  bench("encrypt large (64KB)", () => {
    const cap = new AesCapability(key);
    const payload = Buffer.from("0".repeat(65536));
    cap.encrypt(payload);
  });
});
```

Run benchmarks:

```bash
npx vitest bench tests/bench_aes_throughput.ts
```

### Step 10: Run and Fix

```bash
# Run all tests
npx vitest run

# Run with coverage
npx vitest run --coverage

# Fix failures → re-run → repeat until green
```

### Step 11: Verify Coverage + Perf

```bash
# Check coverage
npx vitest run --coverage --coverage.thresholds.{100,0,0,0}

# Run benchmarks
npx vitest bench tests/bench_<subject>.ts
```

Coverage below threshold → return to Step 2.
Perf regressed > 10% → investigate before merging.

## Quick Reference

```bash
# Run all tests
npx vitest run

# Run one contract test file
npx vitest run tests/contract_aes.ts

# Run one unit test file
npx vitest run tests/unit_aes_encrypt.ts

# Run smoke tests only (filter by name)
npx vitest run --grep "Smoke"

# Run E2E tests only (filter by name)
npx vitest run --grep "E2E"

# Run benchmarks
npx vitest bench tests/bench_aes_throughput.ts

# Check coverage
npx vitest run --coverage

# Run with watch mode
npx vitest
```

## Directory Layout

```
packages/<name>/
├── src/
│   └── capabilities_my_class.ts    # NO inline tests. Clean.
├── tests/
│   ├── contract_aes.ts
│   ├── unit_aes_encrypt.ts
│   ├── unit_aes_decrypt.ts
│   ├── integration_aes.ts
│   ├── smoke_server.ts
│   ├── e2e_encrypt_decrypt_flow.ts
│   ├── acceptance_FRD_042.ts
│   ├── acceptance_FRD_043.ts
│   └── bench_aes_throughput.ts
├── vitest.config.ts                # Test config + coverage targets
└── package.json                    # devDependencies: vitest, ts-jest
```

## Common Mistakes (AVOID)

- ❌ **Testing implementation details**: Test public APIs, not private methods (`#private`)
- ❌ **Missing edge cases**: Always test error paths, invalid inputs, null/undefined values
- ❌ **Over-mocking**: Only mock external dependencies (DB, HTTP), not internal logic
- ❌ **Skipping fixture setup**: Use `beforeEach` / `beforeAll` for consistent test state
- ❌ **Hardcoding assertions**: Use `test.each` for repetitive scenarios
- ❌ **Running benchmarks by default**: Benchmarks should be opt-in (`vitest bench`)

## Vitest Configuration Reference

```typescript
// vitest.config.ts
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    globals: true,           // use global scope (describe, it, expect)
    environment: "node",     // or "jsdom" for browser tests
    include: ["tests/**/*.ts"],
    exclude: ["tests/bench_*.ts"], // benchmarks run separately
    coverage: {
      provider: "v8",        // or "istanbul"
      reporter: ["text", "html"],
      thresholds: {
        statements: 100,
        branches: 100,
        functions: 100,
        lines: 100,
      },
    },
  },
});
```

Run subsets:

```bash
npx vitest run --test-name-pattern "Smoke"   # smoke only
npx vitest run --test-name-pattern "E2E"     # e2e only
npx vitest bench                             # benchmarks only
```
