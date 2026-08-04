/** Utility: Maps SeverityVO to VS Code diagnostic severity. */

import type { DiagnosticSeverity } from "vscode";
import { SeverityVO } from "./taxonomy_severity_vo";

const SEVERITY_MAP: Record<SeverityVO, DiagnosticSeverity> = {
  [SeverityVO.Critical]: 0, // Error
  [SeverityVO.High]: 0, // Error
  [SeverityVO.Medium]: 1, // Warning
  [SeverityVO.Low]: 2, // Information
  [SeverityVO.Info]: 3, // Hint
};

/**
 * Map a SeverityVO to a VS Code DiagnosticSeverity numeric value.
 * Uses raw numbers to avoid importing vscode types in utility layer.
 */
export function mapSeverityToVscode(severity: SeverityVO): number {
  return SEVERITY_MAP[severity] ?? 1;
}
