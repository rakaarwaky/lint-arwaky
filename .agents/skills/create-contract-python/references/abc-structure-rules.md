# ABC Structure Rules

## 1. Contracts contain ABC definitions only

## 2. No default method bodies

## 3. No private helpers or internal stepping stones

## 4. All methods MUST use `@abstractmethod`

## 5. Contracts MUST inherit from `ABC`

## 6. Error strategy

Prefer shared taxonomy error types in contract signatures.

## 7. Protocol naming convention

ABC names MUST use: `I<Name>Port`, `I<Name>Protocol`, `I<Name>Aggregate`
