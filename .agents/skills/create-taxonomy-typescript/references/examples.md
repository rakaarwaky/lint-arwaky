# Examples

## BAD: Interface Defined in Capabilities

```typescript
// capabilities_orphan_analyzer.ts
interface OrphanResult {
    isOrphan: boolean;
    reason: string;
}
```

Fix: Move to taxonomy.

```typescript
// shared/orphan_detector/taxonomy_orphan_result_vo.ts
export interface OrphanResult {
    readonly isOrphan: OrphanFlag;
    readonly reason: OrphanReason;
}
```

## BAD: Taxonomy Importing Layer Code

```typescript
// taxonomy_orphan_vo.ts
import { OrphanAnalyzer } from '../capabilities_orphan_analyzer'; // BAD
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
// shared/orphan_detector/taxonomy_orphan_analysis_result_vo.ts
export interface OrphanAnalysisResult {
    readonly isOrphan: OrphanFlag;
    readonly reason: OrphanReason;
}

// capabilities_orphan_analyzer.ts
export class CapabilitiesOrphanAnalyzer {
    constructor(private readonly extractor: IOrphanFilenameExtractorProtocol) {}
}
```
