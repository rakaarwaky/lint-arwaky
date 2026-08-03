// AES404 test: utility file must not define classes, interfaces, enums, or types
export class ConfigHelper {
    private value: string;
    constructor(v: string) { this.value = v; }
    get(): string { return this.value; }
}
