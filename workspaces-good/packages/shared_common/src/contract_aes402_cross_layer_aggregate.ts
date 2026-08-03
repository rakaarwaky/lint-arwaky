// AES402: contract uses primitive types instead of taxonomy VOs
export interface ConfigProtocol {
    load(id: number, name: string): boolean;
    save(data: Uint8Array): void;
}
