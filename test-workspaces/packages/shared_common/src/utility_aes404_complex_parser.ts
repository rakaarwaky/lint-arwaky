// AES404: utility with complex logic (should be pure function)
export function parseComplex(input: string): string {
    if (!input) {
        throw new Error("empty input");
    }
    
    let result = "";
    for (const ch of input) {
        if (/[a-zA-Z0-9]/.test(ch)) {
            result += ch;
        } else if (ch === ' ') {
            result += '_';
        }
    }
    
    if (result.length > 100) {
        throw new Error("too long");
    }
    
    return result;
}
