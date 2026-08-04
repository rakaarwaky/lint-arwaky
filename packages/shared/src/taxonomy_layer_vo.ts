/** Taxonomy: Layer identification value object for dependency nodes. */

export enum LayerVO {
  Taxonomy = "taxonomy",
  Contract = "contract",
  Utility = "utility",
  Capabilities = "capabilities",
  Agent = "agent",
  Surface = "surface",
  Root = "root",
  Unknown = "unknown",
}

export function layerFromPrefix(filename: string): LayerVO {
  const base = filename.replace(/\.(rs|ts|py|js)$/, "").toLowerCase();
  if (base.startsWith("taxonomy_")) return LayerVO.Taxonomy;
  if (base.startsWith("contract_")) return LayerVO.Contract;
  if (base.startsWith("utility_")) return LayerVO.Utility;
  if (base.startsWith("capabilities_") || base.startsWith("capability_")) return LayerVO.Capabilities;
  if (base.startsWith("agent_")) return LayerVO.Agent;
  if (base.startsWith("surface_")) return LayerVO.Surface;
  if (base.startsWith("root_")) return LayerVO.Root;
  return LayerVO.Unknown;
}

export function layerLabel(layer: LayerVO): string {
  return layer.charAt(0).toUpperCase() + layer.slice(1);
}
