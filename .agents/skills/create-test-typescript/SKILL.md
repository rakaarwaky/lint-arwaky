---
name: create-test-typescript
description: "Create comprehensive test suites for TypeScript modules following Vitest/Jest conventions and project testing standards."
metadata:
  tags: [typescript, testing, vitest, jest, mocking, fixtures]
  triggers:
    - "create test typescript"
    - "add tests typescript"
    - "write unit tests typescript"
  dependencies: []
  related:
    - add-docs-typescript
    - cleanup-files-typescript
---

# create-test-typescript

## Purpose

Create comprehensive test suites for TypeScript modules following Vitest/Jest conventions and project testing standards. Ensures all public APIs are tested with proper fixtures and mocking.

## Rules

### Test Structure

- Use `vitest` or `jest` framework
- Name test files: `*.test.ts` or `*.spec.ts`
- Name test functions: `test_<function>_<scenario>` or `it('<description>')`
- Place tests in same directory structure as source

### Testing Conventions

- Use `expect` assertions
- Use `describe` blocks for grouping
- Mock external dependencies with `vi.fn()` or `jest.fn()`
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
grep -rn "^class \|^function \|^export " packages/*/src/ | grep -v "^_"
```

### Step 2: Create Test File

Create `*.test.ts` with test structure:

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { MyClass } from "../src/capabilities_my_class";

describe("MyClass", () => {
  let mockDep: ReturnType<typeof vi.fn>;
  let testObj: MyClass;

  beforeEach(() => {
    mockDep = vi.fn();
    testObj = new MyClass(mockDep);
  });

  it("should return expected value", () => {
    const result = testObj.method();
    expect(result).toBe(expectedValue);
  });

  it("should throw on invalid input", () => {
    expect(() => testObj.method(invalidInput)).toThrow(ValueError);
  });
});
```

### Step 3: Add Fixtures

Create shared fixtures for test data:

```typescript
// fixtures.ts
export const createMockFile = (path: string, content: string) => ({
  path,
  content,
  size: content.length,
});
```

### Step 4: Mock External Dependencies

Use `vi.fn()` or `jest.fn()` to isolate tests:

```typescript
import { vi, describe, it, expect } from "vitest";
import { db } from "../src/infrastructure_db";

vi.mock("../src/infrastructure_db", () => ({
  db: {
    connect: vi.fn(),
    query: vi.fn(),
  },
}));

describe("Database Operation", () => {
  it("should connect to database", async () => {
    vi.mocked(db.connect).mockResolvedValue(true);
    // ... test logic
  });
});
```

### Step 5: Run Tests

```bash
# Run all tests
npx vitest run

# Run with coverage
npx vitest run --coverage

# Run in watch mode
npx vitest
```

## Verification Checklist

- [ ] All public APIs have test coverage
- [ ] Test files follow naming conventions (`*.test.ts`)
- [ ] Fixtures used for setup/teardown
- [ ] External dependencies mocked appropriately
- [ ] Edge cases and error paths tested
- [ ] Tests run successfully with vitest

## Quick Commands

```bash
# Run all tests
npx vitest run

# Run with coverage report
npx vitest run --coverage

# Find untested files
find packages/*/src/ -name "*.ts" ! -path "*test*" | while read f; do
    base=$(basename "$f" .ts)
    grep -rl "from.*$base\|import.*$base" packages/*/src/*.test.ts || echo "UNTESTED: $base"
done

# Check test coverage
npx vitest run --coverage
```

## Common Mistakes (AVOID)

- ❌ **Testing implementation details**: Test public APIs, not internal methods
- ❌ **Missing edge cases**: Always test error paths and invalid inputs
- ❌ **Over-mocking**: Only mock external dependencies, not internal logic
- ❌ **Skipping fixture setup**: Use fixtures for consistent test state
