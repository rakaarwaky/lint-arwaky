// BAD: Interface defined in capabilities layer (AES201)
interface <NameResult> {
    is_valid: boolean;
    reason: string;
}

export class Capabilities<NameCapability> {
    execute(): <NameResult> {
        return { is_valid: true, reason: '' };
    }
}
