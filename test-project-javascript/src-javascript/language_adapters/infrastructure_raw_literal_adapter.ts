// PURPOSE: Test AES0304 — raw literals in infrastructure
export class RawLiteralAdapter {
    timeout: number = 30;
    host: string = "localhost";
    port: number = 8080;

    connect(): string {
        const url = "http://example.com";
        const retries = 3;
        return url;
    }
}
