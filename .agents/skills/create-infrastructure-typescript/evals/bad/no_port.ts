// BAD: No port interface implementation (AES404)
export class FileCache {
    read(): string {
        // public behavior without port interface
        return '';
    }
}
