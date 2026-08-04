/** Contract: Graph builder protocol — implemented by Capabilities layer. */

import { ViolationVO } from "./taxonomy_violation_vo";
import { DependencyGraphVO } from "./taxonomy_dependency_graph_vo";

export interface IGraphBuilderProtocol {
  buildGraphFromViolations(violations: readonly ViolationVO[]): DependencyGraphVO;
}
