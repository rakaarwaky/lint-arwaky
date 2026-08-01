// AES402: Contract protocol with forbidden primitive types in method signatures
export interface PrimitiveProtocol {
    getValue(): number;
    setName(name: string): void;
}
