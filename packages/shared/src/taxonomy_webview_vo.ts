/** Taxonomy: Extension ↔ webview message protocol types.

Discriminated unions for type-safe communication.
*/

import { SeverityVO } from "./taxonomy_severity_vo";
import { ViolationVO } from "./taxonomy_violation_vo";
import { DependencyGraphVO } from "./taxonomy_dependency_graph_vo";

// ─── Extension → Webview ──────────────────────────────

export interface ShowDependencyGraphMessage {
  command: "showDependencyGraph";
  graph: DependencyGraphVO;
}

export interface ShowViolationsMessage {
  command: "showViolations";
  violations: readonly ViolationVO[];
}

export interface ScanProgressMessage {
  command: "scanProgress";
  status: "scanning" | "complete" | "error";
  message?: string;
}

export interface EmptyStateMessage {
  command: "emptyState";
  reason: "no-workspace" | "no-violations" | "scan-failed";
}

export type ExtensionToWebviewMessage =
  | ShowDependencyGraphMessage
  | ShowViolationsMessage
  | ScanProgressMessage
  | EmptyStateMessage;

// ─── Webview → Extension ──────────────────────────────

export interface RefreshScanMessage {
  command: "refreshScan";
}

export interface OpenViolationMessage {
  command: "openViolation";
  file: string;
  line: number;
  column: number;
}

export interface FilterSeverityMessage {
  command: "filterSeverity";
  severity: SeverityVO;
}

export interface FilterLayerMessage {
  command: "filterLayer";
  layer: string;
}

export type WebviewToExtensionMessage =
  | RefreshScanMessage
  | OpenViolationMessage
  | FilterSeverityMessage
  | FilterLayerMessage;
