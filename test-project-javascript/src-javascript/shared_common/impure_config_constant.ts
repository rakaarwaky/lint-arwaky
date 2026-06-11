export const MAX_RETRIES: number = 3;
export const TIMEOUT_MS: number = 5000;
export class ImpureConfig {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}
