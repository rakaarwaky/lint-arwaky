// BAD: Taxonomy importing layer code (AES201)
import { OrphanAnalyzer } from '../capabilities_orphan_analyzer'; // BAD

export interface OrphanResult {
    readonly isOrphan: boolean;
    readonly reason: string;
}
