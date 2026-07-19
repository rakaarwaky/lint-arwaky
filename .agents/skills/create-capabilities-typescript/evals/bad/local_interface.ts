// BAD: Interface defined in capabilities layer (AES201)
interface OrphanResult {
    isOrphan: boolean;
    reason: string;
}

class CapabilitiesOrphanAnalyzer {
    analyze(): OrphanResult {
        return { isOrphan: true, reason: '' };
    }
}
