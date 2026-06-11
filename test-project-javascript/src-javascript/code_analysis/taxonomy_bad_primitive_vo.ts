// AES0301 — direct primitive in taxonomy test
// Taxonomy files should use VOs, not raw primitives
export class BadPrimitiveVO {
  name: string = "";
  count: number = 0;
  enabled: boolean = false;
  items: Array<string> = [];
}
