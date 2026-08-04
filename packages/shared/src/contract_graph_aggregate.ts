/** Contract: Graph aggregate — used by Surface layer to access graph data. */

import { ScanRequestVO } from "./taxonomy_scan_request_vo";
import { DependencyGraphVO } from "./taxonomy_dependency_graph_vo";

export interface IGraphAggregate {
  execute(request: ScanRequestVO): Promise<DependencyGraphVO>;
}
