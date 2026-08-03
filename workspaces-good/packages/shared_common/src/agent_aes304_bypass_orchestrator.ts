// AES304: bypass annotation - throw new Error() pattern
export class BypassEntity {
    unsafe_method(): void {
        throw new Error("bypass"); // AES304: forbidden throw pattern
    }
}
