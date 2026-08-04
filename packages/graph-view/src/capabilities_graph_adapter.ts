/** Capabilities: Graph aggregate — implements IGraphAggregate.

Combines IScannerProtocol (scan violations) + IGraphBuilderProtocol (build graph)
to fulfill the IGraphAggregate contract used by the Surface layer.
*/

import { IGraphAggregate } from "@lint-arwaky/shared/src/contract_graph_aggregate";
import { IScannerProtocol } from "@lint-arwaky/shared/src/contract_scanner_protocol";
import { IGraphBuilderProtocol } from "@lint-arwaky/shared/src/contract_graph_builder_protocol";
import { ScanRequestVO } from "@lint-arwaky/shared/src/taxonomy_scan_request_vo";
import { DependencyGraphVO } from "@lint-arwaky/shared/src/taxonomy_dependency_graph_vo";

// ─── Block 1: Class Definition & Constructor ──────────────

export class GraphAggregateAdapter implements IGraphAggregate {
  constructor(
    private readonly scanner: IScannerProtocol,
    private readonly graphBuilder: IGraphBuilderProtocol,
  ) {}

  // ─── Block 2: Aggregate Method Implementation ─────────────

  async execute(request: ScanRequestVO): Promise<DependencyGraphVO> {
    const result = await this.scanner.scan(request);
    return this.graphBuilder.buildGraphFromViolations(result.violations);
  }
}
