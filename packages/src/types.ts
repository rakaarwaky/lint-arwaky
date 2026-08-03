export interface ScanResult {
  members: MemberSummary[];
  results: Violation[];
}

export interface MemberSummary {
  member: string;
  violations: number;
}

export interface Violation {
  code: string;
  column: number;
  file: string;
  line: number;
  member: string;
  message: string;
  severity: string;
}
