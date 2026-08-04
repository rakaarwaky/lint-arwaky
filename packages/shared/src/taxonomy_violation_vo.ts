/** Taxonomy: Violation value object — single lint violation reported by scanner. */

import { SeverityVO, severityFromValue } from "./taxonomy_severity_vo";

export interface ViolationVO {
  readonly code: string;
  readonly column: number;
  readonly file: string;
  readonly line: number;
  readonly member: string;
  readonly message: string;
  readonly severity: SeverityVO;
}

export function createViolation(raw: {
  code: string;
  column: number;
  file: string;
  line: number;
  member: string;
  message: string;
  severity: string;
}): ViolationVO {
  return {
    code: raw.code,
    column: raw.column,
    file: raw.file,
    line: raw.line,
    member: raw.member,
    message: raw.message,
    severity: severityFromValue(raw.severity),
  };
}
