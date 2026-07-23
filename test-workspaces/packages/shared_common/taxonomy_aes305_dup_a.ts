export class DupEntityA {
    do_something(): string {
        const x = 1;
        const y = 2;
        const z = x + y;
        return `result: ${z}`;
    }
}
