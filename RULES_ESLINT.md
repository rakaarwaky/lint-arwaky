# ESLint Linting Rules (JavaScript/TypeScript)

ESLint is the standard linter for JavaScript and TypeScript, available at https://eslint.org/. It enforces code quality and style rules.

## Rule Categories

| Prefix | Category    | Description                             |
| ------ | ----------- | --------------------------------------- |
| error  | Error       | Rules that produce errors when violated |
| warn   | Warning     | Rules that produce warnings             |
| off    | Disabled    | Rules that are turned off               |
| S      | Security    | Security-related rules                  |
| D      | Debug/Dev   | Development-only rules                  |
| P      | Performance | Performance-related rules               |

## Key Rules

| Code                  | Rule                                                   | Severity |
| --------------------- | ------------------------------------------------------ | -------- |
| no-unused-vars        | Variables, functions, or imports declared but not used | Error    |
| no-undef              | Use of undefined variable                              | Error    |
| no-console            | Use of `console.log()` left in production code         | Warning  |
| no-debugger           | Use of `debugger` statement left in code               | Error    |
| no-eval               | Use of `eval()` which executes arbitrary code          | Error    |
| no-implied-eval       | Use of `setTimeout`/`setInterval` with string argument | Error    |
| no-new-func           | Use of `new Function()` which creates eval-like code   | Error    |
| no-param-reassign     | Reassigning function parameters (side effects)         | Error    |
| no-var                | Use of `var` instead of `let` or `const`               | Error    |
| prefer-const          | Variable never reassigned should be `const`            | Error    |
| prefer-template       | String concatenation should use template literals      | Warning  |
| arrow-body-style      | Arrow function body style inconsistency                | Warning  |
| no-shadow             | Variable shadows outer scope variable                  | Error    |
| max-len               | Line exceeds configured max length                     | Warning  |
| max-params            | Function exceeds maximum parameters                    | Warning  |
| max-depth             | Block nesting exceeds maximum depth                    | Warning  |
| complexity            | Function exceeds cyclomatic complexity limit           | Warning  |
| no-duplicate-imports  | Duplicate imports from same module                     | Error    |
| sort-imports          | Import statements not sorted                           | Warning  |
| no-restricted-imports | Import from restricted modules (e.g., deprecated libs) | Error    |
| no-restricted-globals | Use of restricted global variables                     | Error    |
| no-process-env        | Direct use of `process.env` (should use config module) | Warning  |
| camelcase             | Non-camelCase variable naming                          | Error    |
| new-cap               | Constructor name not starting with uppercase           | Error    |
| no-empty              | Empty block statement (if/while/for without body)      | Warning  |
| no-extra-boolean-cast | Unnecessary boolean cast (`!!x` when `x` is boolean)   | Warning  |
| eqeqeq                | Use of `==` instead of `===`                           | Error    |
| curly                 | Missing curly braces around blocks                     | Warning  |
| dot-notation          | Property access via bracket notation when dot works    | Warning  |
| guard-for-in          | `for-in` loop without `hasOwnProperty` check           | Error    |
| no-fallthrough        | Fallthrough in switch case without explicit comment    | Error    |
| no-redeclare          | Variable redeclared in same scope                      | Error    |
| no-return-assign      | Assignment in return statement                         | Warning  |
| no-throw-literal      | Throwing non-Error object (`throw "string"`)           | Error    |
| no-unused-expressions | Expression without effect (e.g., bare string)          | Error    |
| no-useless-catch      | Catch block that only re-throws the error              | Warning  |
| require-await         | Async function without `await` inside body             | Warning  |
| valid-typeof          | Typeof comparison against invalid string               | Error    |

## TypeScript-Specific Rules

| Code                                             | Rule                                                | Severity |
| ------------------------------------------------ | --------------------------------------------------- | -------- | ------------------------ | ------- |
| @typescript-eslint/no-explicit-any               | Use of `any` type bypasses type safety              | Error    |
| @typescript-eslint/no-unused-vars                | Variable/parameter declared but not used            | Error    |
| @typescript-eslint/explicit-function-return-type | Function missing return type annotation             | Warning  |
| @typescript-eslint/no-non-null-assertion         | Use of non-null assertion `!` operator              | Warning  |
| @typescript-eslint/no-unsafe-assignment          | Assignment of `any` typed value to typed variable   | Error    |
| @typescript-eslint/no-unsafe-call                | Calling an `any` typed value                        | Error    |
| @typescript-eslint/no-unsafe-member-access       | Accessing property on `any` typed value             | Error    |
| @typescript-eslint/no-unsafe-return              | Returning `any` from typed function                 | Error    |
| @typescript-eslint/ban-types                     | Use of banned types (`String`, `Number`, `Boolean`) | Error    |
| @typescript-eslint/prefer-optional-chain         | Nested `&&` for optional access vs `?.`             | Warning  |
| @typescript-eslint/prefer-nullish-coalescing     | `                                                   |          | `with falsy check vs`??` | Warning |
