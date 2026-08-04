/** Taxonomy: Member summary value object — violation count per workspace member. */

export interface MemberSummaryVO {
  readonly member: string;
  readonly violations: number;
}

export function createMemberSummary(
  member: string,
  violations: number,
): MemberSummaryVO {
  return { member, violations };
}
