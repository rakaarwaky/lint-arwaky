/** Taxonomy: Dependency graph value object — complete graph for webview rendering. */

import { DependencyNodeVO } from "./taxonomy_dependency_node_vo";
import { DependencyEdgeVO } from "./taxonomy_dependency_edge_vo";
import { ViolationVO } from "./taxonomy_violation_vo";

export interface DependencyGraphVO {
  readonly nodes: readonly DependencyNodeVO[];
  readonly edges: readonly DependencyEdgeVO[];
  readonly violations: readonly ViolationVO[];
}

export function createDependencyGraph(
  nodes: readonly DependencyNodeVO[],
  edges: readonly DependencyEdgeVO[],
  violations: readonly ViolationVO[],
): DependencyGraphVO {
  return { nodes, edges, violations };
}
