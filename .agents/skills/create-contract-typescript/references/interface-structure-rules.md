# Interface Structure Rules

## 1. Contracts contain interface definitions only

## 2. No method implementations

## 3. No private helper signatures

## 4. All methods MUST have proper type annotations

## 5. Contracts MUST be exported with `export interface`

## 6. Error strategy

Prefer shared taxonomy error types in contract signatures.

## 7. Interface naming convention

Interface names MUST use: `I<Name>Port`, `I<Name>Protocol`, `I<Name>Aggregate`
