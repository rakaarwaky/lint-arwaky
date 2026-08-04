/** Taxonomy: Dependency node value object — represents a file/module in the graph. */

import { LayerVO, layerFromPrefix } from "./taxonomy_layer_vo";

export interface DependencyNodeVO {
  readonly id: string;
  readonly label: string;
  readonly layer: LayerVO;
  readonly violationCount: number;
  readonly highestSeverity: string;
}

export function createDependencyNode(
  id: string,
  violationCount: number,
  highestSeverity: string,
): DependencyNodeVO {
  const parts = id.replace(/\\/g, "/").split("/");
  const filename = parts[parts.length - 1] || id;
  return {
    id,
    label: filename,
    layer: layerFromPrefix(filename),
    violationCount,
    highestSeverity,
  };
}
