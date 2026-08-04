/** Utility: Maps SeverityVO to VS Code diagnostic severity integers.

Stateless, pure, domain-agnostic mapping function.
DiagnosticSeverity enum values: Error=0, Warning=1, Information=2, Hint=3.
*/

import { SeverityVO } from "./taxonomy_severity_vo";

/** Map a SeverityVO to VS Code DiagnosticSeverity integer value. */
export function mapSeverityToVscode(severity: SeverityVO): number {
  switch (severity) {
    case SeverityVO.Critical:
      return 0; // Error
    case SeverityVO.High:
      return 0; // Error
    case SeverityVO.Medium:
      return 1; // Warning
    case SeverityVO.Low:
      return 2; // Information
    case SeverityVO.Info:
      return 3; // Hint
    default:
      return 1; // Warning
  }
}
