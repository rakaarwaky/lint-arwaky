# BAD: I/O in capabilities layer (AES404)
export class <NameCapability> {
    process(): void {
        const content = fs.readFileSync('file.txt', 'utf-8'); // FORBIDDEN
    }
}
