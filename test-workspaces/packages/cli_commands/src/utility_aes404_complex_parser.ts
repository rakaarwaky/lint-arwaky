// AES404: utility contains forbidden type definition (interface)
export interface ParserConfig {
    maxDepth: number;
    strict: boolean;
}

export function parse(input: string): string {
    return input.toUpperCase();
}
