# Examples

## BAD: Interface Defined in Capabilities

```typescript
// capabilities_<name-capability>.ts
interface <ResultVO> {
    is_valid: boolean;
    reason: string;
}
```

Fix: Move to taxonomy.

```typescript
// shared/<name-feature>/taxonomy_<name>_result_vo.ts
export interface <ResultVO> {
    readonly is_valid: <Flag>VO;
    readonly reason: <Reason>VO;
}
```

## BAD: Taxonomy Importing Layer Code

```typescript
// taxonomy_<name>_vo.ts
import { <NameAnalyzer> } from '../capabilities_<name-capability>'; // BAD
```

Taxonomy must not import from layers.

## BAD: Domain Rule Inside Utility

```typescript
export function isPortTraitName(name: string): boolean {
    return name.endsWith('Port');
}
```

If this knows AES naming conventions, it belongs in capabilities as a helper.

## GOOD: Interface in Taxonomy + Implementor with DI

```typescript
// shared/<name-feature>/taxonomy_<name>_analysis_result_vo.ts
export interface <AnalysisResult>VO {
    readonly is_valid: <Flag>VO;
    readonly reason: <Reason>VO;
}

// capabilities_<name-capability>.ts
export class <NameCapability> {
    constructor(private readonly collaborator: I<NameCollaborator>Protocol) {}
}
```
