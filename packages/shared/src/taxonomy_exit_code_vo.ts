/** Taxonomy: Exit code value object for CLI execution results. */

export interface ExitCodeVO {
  readonly value: number | null;
}

export function createExitCode(value: number | null): ExitCodeVO {
  return { value };
}
