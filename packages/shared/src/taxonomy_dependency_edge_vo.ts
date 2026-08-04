/** Taxonomy: Dependency edge value object — represents an import/dependency between files. */

export interface DependencyEdgeVO {
  readonly id: string;
  readonly source: string;
  readonly target: string;
  readonly type: "import" | "dependency";
}

export function createDependencyEdge(
  source: string,
  target: string,
  type: "import" | "dependency" = "import",
): DependencyEdgeVO {
  return {
    id: `${source}->${target}`,
    source,
    target,
    type,
  };
}
