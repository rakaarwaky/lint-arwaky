/** Taxonomy: CLI execution error — wraps lint-arwaky-cli failures. */

export class CliError extends Error {
  readonly exitCode: number | null;
  readonly stderr: string;

  constructor(message: string, exitCode: number | null, stderr: string) {
    super(message);
    this.name = "CliError";
    this.exitCode = exitCode;
    this.stderr = stderr;
  }
}
