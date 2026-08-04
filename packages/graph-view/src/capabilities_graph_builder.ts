/** Capabilities: Dependency graph builder — implements IGraphBuilderProtocol.

Block 1: Class Definition & Constructor
Block 2: Protocol Method Implementation
Block 3: Utility Methods, Factories & Helpers
*/

import { ViolationVO } from "@lint-arwaky/shared/src/taxonomy_violation_vo";
import { DependencyGraphVO, createDependencyGraph } from "@lint-arwaky/shared/src/taxonomy_dependency_graph_vo";
import { DependencyNodeVO, createDependencyNode } from "@lint-arwaky/shared/src/taxonomy_dependency_node_vo";
import { DependencyEdgeVO, createDependencyEdge } from "@lint-arwaky/shared/src/taxonomy_dependency_edge_vo";
import { IGraphBuilderProtocol } from "@lint-arwaky/shared/src/contract_graph_builder_protocol";

// ─── Block 1: Class Definition & Constructor ──────────────
export class DependencyGraphBuilder implements IGraphBuilderProtocol {
  // ─── Block 2: Protocol Method Implementation ─────────────
  buildGraphFromViolations(violations: readonly ViolationVO[]): DependencyGraphVO {
    const nodes = this.buildNodes(violations);
    const edges = this.buildEdges(violations);
    return createDependencyGraph(nodes, edges, violations);
  }

  // ─── Block 3: Utility Methods, Factories & Helpers ────────
  private buildNodes(violations: readonly ViolationVO[]): DependencyNodeVO[] {
    const nodeMap = new Map<string, { count: number; severity: string }>();

    for (const v of violations) {
      const existing = nodeMap.get(v.file);
      if (existing) {
        existing.count++;
        if (this.severityRank(v.severity) > this.severityRank(existing.severity)) {
          existing.severity = v.severity;
        }
      } else {
        nodeMap.set(v.file, { count: 1, severity: v.severity });
      }
    }

    return Array.from(nodeMap.entries()).map(([file, data]) =>
      createDependencyNode(file, data.count, data.severity),
    );
  }

  private buildEdges(violations: readonly ViolationVO[]): DependencyEdgeVO[] {
    const edgeSet = new Set<string>();
    const edges: DependencyEdgeVO[] = [];

    for (const v of violations) {
      const targetNode = v.member;
      if (targetNode && targetNode !== v.file) {
        const key = `${v.file}->${targetNode}`;
        if (!edgeSet.has(key)) {
          edgeSet.add(key);
          edges.push(createDependencyEdge(v.file, targetNode));
        }
      }
    }

    return edges;
  }

  private severityRank(severity: string): number {
    switch (severity) {
      case "critical": return 5;
      case "high": return 4;
      case "medium": return 3;
      case "low": return 2;
      case "info": return 1;
      default: return 0;
    }
  }
}
