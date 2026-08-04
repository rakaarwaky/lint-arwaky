/** Taxonomy: CLI execution error — wraps lint-arwaky-cli failures.

ExitCodeVO and StderrVO live in taxonomy_exit_code_vo.ts and taxonomy_stderr_vo.ts
to avoid AES401 primitives in error taxonomy files.
*/

import { ExitCodeVO, createExitCode } from "./taxonomy_exit_code_vo";
import { StderrVO, createStderr } from "./taxonomy_stderr_vo";

export type { ExitCodeVO } from "./taxonomy_exit_code_vo";
export type { StderrVO } from "./taxonomy_stderr_vo";

export class CliError extends Error {
  readonly exitCode: ExitCodeVO;
  readonly stderr: StderrVO;

  constructor(message: string, exitCode: number | null, stderr: string) {
    super(message);
    this.name = "CliError";
    this.exitCode = createExitCode(exitCode);
    this.stderr = createStderr(stderr);
  }
}
