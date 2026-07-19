export class FilePath {
    private readonly _value: string;

    constructor(value: string) {
        if (!value.trim()) {
            throw new Error('FilePath cannot be empty');
        }
        this._value = value;
    }

    get value(): string {
        return this._value;
    }

    toString(): string {
        return this._value;
    }
}
