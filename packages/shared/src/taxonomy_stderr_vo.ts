/** Taxonomy: Stderr value object for CLI execution results. */

export interface StderrVO {
  readonly value: string;
}

export function createStderr(value: string): StderrVO {
  return { value };
}
