// BAD: Computation in agent layer
export class OrphanOrchestrator {
    process(files: FilePath[]) {
        const total = files.length; // BAD: computation
        const sum = files.reduce((acc, f) => acc + f.size, 0); // BAD
    }
}
