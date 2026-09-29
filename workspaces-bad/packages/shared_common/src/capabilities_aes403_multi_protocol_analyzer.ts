// AES403 multi-protocol: a single capability implements two interfaces.
export class MultiProtocolAnalyzer implements IAlphaProtocol, IBetaProtocol {
    execute(): number {
        return 42;
    }
}
