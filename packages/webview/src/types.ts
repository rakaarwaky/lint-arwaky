/** Passive Surface: Types for webview — mirrors taxonomy VOs. */

export interface ViolationVO {
  readonly code: string;
  readonly column: number;
  readonly file: string;
  readonly line: number;
  readonly member: string;
  readonly message: string;
  readonly severity: string;
}

export interface DependencyNodeVO {
  readonly id: string;
  readonly label: string;
  readonly layer: string;
  readonly violationCount: number;
  readonly highestSeverity: string;
}

export interface DependencyEdgeVO {
  readonly id: string;
  readonly source: string;
  readonly target: string;
  readonly type: string;
}

export interface DependencyGraphVO {
  readonly nodes: readonly DependencyNodeVO[];
  readonly edges: readonly DependencyEdgeVO[];
  readonly violations: readonly ViolationVO[];
}

export interface ScanProgressMessage {
  command: 'scanProgress';
  status: 'scanning' | 'complete' | 'error';
  message?: string;
}

export interface ShowDependencyGraphMessage {
  command: 'showDependencyGraph';
  graph: DependencyGraphVO;
}

export interface EmptyStateMessage {
  command: 'emptyState';
  reason: 'no-workspace' | 'no-violations' | 'scan-failed';
  message?: string;
}

export type ExtensionToWebviewMessage =
  | ScanProgressMessage
  | ShowDependencyGraphMessage
  | EmptyStateMessage;
