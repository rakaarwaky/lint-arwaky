export class <Name> {
    private readonly _value: string;

    constructor(value: string) {
        if (!value.trim()) {
            throw new Error('<Name> cannot be empty');
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
