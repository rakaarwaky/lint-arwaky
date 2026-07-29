import { IOrphanAnalyzerProtocol } from '../shared/orphan_detector/contract_orphan_protocol';
import { IOrphanOrchestratorAggregate } from '../shared/orphan_detector/contract_orphan_aggregate';
import { LintResult } from '../shared/code_analysis/taxonomy_result_vo';

// ─── Block 1: Class Definition & Constructor ──────────────
export class OrphanOrchestrator implements IOrphanOrchestratorAggregate {
    constructor(private readonly analyzer: IOrphanAnalyzerProtocol) {}

    // ─── Block 2: Public Contract (domain aggregate ONLY) ──
    execute(request: ScanRequest): LintResult[] {
        const violations: LintResult[] = [];
        for (const file of request.files()) {
            try {
                const result = this.analyzer.analyze(file);
                violations.push(...result.intoViolations());
            } catch (err) {
                violations.push(LintResult.fromAnalysisError(file, err));
            }
        }
        return violations;
    }

    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return 'OrphanOrchestrator()';
    }

    static create(): OrphanOrchestrator {
        return new OrphanOrchestrator(new CapabilitiesOrphanAnalyzer());
    }
}
