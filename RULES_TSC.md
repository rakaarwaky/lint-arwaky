# TypeScript Compiler (TSC) Rules

TypeScript is the type-checked superset of JavaScript, available at https://www.typescriptlang.org/. The TypeScript compiler (`tsc`) enforces type safety and modern JavaScript features.

## Strict Mode Checks

When `strict: true` is enabled in `tsconfig.json`, these checks are all active:

| Check                        | Description                                              | Severity |
| ---------------------------- | -------------------------------------------------------- | -------- |
| strictNullChecks             | `null` and `undefined` are not assignable to other types | Error    |
| strictFunctionTypes          | Function parameter bivariance is disabled                | Error    |
| strictBindCallApply          | `bind`, `call`, `apply` are properly typed               | Error    |
| strictPropertyInitialization | Class properties must be initialized in constructor      | Error    |
| noImplicitAny                | Expressions or declarations with implied `any` type      | Error    |
| noImplicitThis               | `this` expression with implied `any` type                | Error    |
| alwaysStrict                 | Code emitted in strict mode                              | Error    |
| useUnknownInCatchVariables   | Catch clause variable defaults to `unknown`              | Error    |

## Key Compiler Options

| Option                             | Description                                     | Impact |
| ---------------------------------- | ----------------------------------------------- | ------ |
| noUnusedLocals                     | Report errors on unused local variables         | Error  |
| noUnusedParameters                 | Report errors on unused function parameters     | Error  |
| noImplicitReturns                  | Report error when function path has no return   | Error  |
| noFallthroughCasesInSwitch         | Report fallthrough in switch statements         | Error  |
| exactOptionalPropertyTypes         | Optional properties treated as `T\|undefined`   | Error  |
| noUncheckedIndexedAccess           | Accessing index type adds `undefined` to result | Error  |
| noPropertyAccessFromIndexSignature | Report access to index signature via dot syntax | Error  |
| forceConsistentCasingInFileNames   | Disallow inconsistently-cased file references   | Error  |

## Type Checking Rules

| Pattern | Rule                                   | Description                                     |
| ------- | -------------------------------------- | ----------------------------------------------- |
| TS2304  | Cannot find name                       | Variable/type used without declaration          |
| TS2322  | Type not assignable                    | Value assigned to incompatible type             |
| TS2339  | Property does not exist                | Accessing property not on type                  |
| TS2345  | Argument not assignable                | Wrong argument type to function                 |
| TS2352  | Conversion type error                  | Invalid type assertion                          |
| TS2362  | Numeric comparison                     | Comparing number to incompatible type           |
| TS2367  | Comparison appears to be unintentional | Conditions that are always true/false           |
| TS2531  | Object possibly null                   | Accessing property on possibly null object      |
| TS2532  | Object possibly undefined              | Accessing property on possibly undefined object |
| TS2540  | Cannot assign to read-only property    | Modifying `readonly` property                   |
| TS2554  | Expected N arguments but got M         | Wrong number of function arguments              |
| TS2564  | Property not initialized               | Class property not set in constructor           |
| TS2578  | Unused '@ts-expect-error' directive    | Directive with no matching error                |
| TS2589  | Type instantiation is excessively deep | Generic type too complex                        |
| TS2740  | Type missing properties                | Object literal missing required properties      |
| TS2741  | Property is missing in type            | Object missing required property                |
| TS2769  | No overload matches                    | Call does not match any overload signature      |
| TS2790  | Operand of 'delete' must be optional   | Using `delete` on required property             |
| TS2869  | Interface name conflicts               | Interface name collides with type alias         |

## Common Best Practice Config

```json
{
  "compilerOptions": {
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true,
    "exactOptionalPropertyTypes": true,
    "noUncheckedIndexedAccess": true,
    "forceConsistentCasingInFileNames": true
  }
}
```
