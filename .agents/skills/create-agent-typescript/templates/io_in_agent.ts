// BAD: I/O in agent layer
export class OrphanOrchestrator {
    execute(path: FilePath) {
        const content = fs.readFileSync(path.value()); // BAD
    }
}
