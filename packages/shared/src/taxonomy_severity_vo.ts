/** Taxonomy: Severity level value object. */

export enum SeverityVO {
  Critical = "critical",
  High = "high",
  Medium = "medium",
  Low = "low",
  Info = "info",
}

export function severityFromValue(value: string): SeverityVO {
  const normalized = value.toLowerCase() as SeverityVO;
  if (
    normalized === SeverityVO.Critical ||
    normalized === SeverityVO.High ||
    normalized === SeverityVO.Medium ||
    normalized === SeverityVO.Low ||
    normalized === SeverityVO.Info
  ) {
    return normalized;
  }
  return SeverityVO.Medium;
}
