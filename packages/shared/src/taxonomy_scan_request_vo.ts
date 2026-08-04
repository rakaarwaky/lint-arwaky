/** Taxonomy: Scan request value object — encapsulates a scan invocation. */

export interface ScanRequestVO {
  readonly targetPath: string;
  readonly language?: string;
}

export function createScanRequest(
  targetPath: string,
  language?: string,
): ScanRequestVO {
  return { targetPath, language };
}
