/** Taxonomy: Scan result value object — aggregate output of a lint scan. */

import { ViolationVO } from "./taxonomy_violation_vo";
import { MemberSummaryVO } from "./taxonomy_member_summary_vo";

export interface ScanResultVO {
  readonly members: readonly MemberSummaryVO[];
  readonly violations: readonly ViolationVO[];
}

export function createScanResult(
  members: readonly MemberSummaryVO[],
  violations: readonly ViolationVO[],
): ScanResultVO {
  return { members, violations };
}
