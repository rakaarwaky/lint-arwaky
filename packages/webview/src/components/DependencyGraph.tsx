/** Passive Surface: Dependency graph visualization component.

Renders nodes and edges from DependencyGraphVO.
Uses SVG for simple graph visualization.
*/

import React, { useMemo } from 'react';
import { DependencyGraphVO, DependencyNodeVO } from '../types';
import { LayerBadge } from './LayerBadge';

interface DependencyGraphProps {
  graph: DependencyGraphVO;
  filterSeverity: string | null;
  filterLayer: string | null;
}

const SEVERITY_COLORS: Record<string, string> = {
  critical: '#f44336',
  high: '#ff5722',
  medium: '#ff9800',
  low: '#4caf50',
  info: '#2196f3',
};

const LAYER_COLORS: Record<string, string> = {
  taxonomy: '#9c27b0',
  contract: '#3f51b5',
  utility: '#00bcd4',
  capabilities: '#ff9800',
  agent: '#e91e63',
  surface: '#4caf50',
  root: '#f44336',
  unknown: '#757575',
};

function getNodeColor(node: DependencyNodeVO): string {
  return SEVERITY_COLORS[node.highestSeverity] || '#757575';
}

function getNodeBorder(node: DependencyNodeVO): string {
  return LAYER_COLORS[node.layer] || '#757575';
}

export function DependencyGraph({ graph, filterSeverity, filterLayer }: DependencyGraphProps) {
  const filteredNodes = useMemo(() => {
    return graph.nodes.filter(node => {
      if (filterSeverity && node.highestSeverity !== filterSeverity) return false;
      if (filterLayer && node.layer !== filterLayer) return false;
      return true;
    });
  }, [graph.nodes, filterSeverity, filterLayer]);

  const nodeMap = useMemo(() => {
    const map = new Map<string, DependencyNodeVO>();
    for (const node of filteredNodes) {
      map.set(node.id, node);
    }
    return map;
  }, [filteredNodes]);

  const filteredEdges = useMemo(() => {
    return graph.edges.filter(edge => {
      return nodeMap.has(edge.source) && nodeMap.has(edge.target);
    });
  }, [graph.edges, nodeMap]);

  if (filteredNodes.length === 0) {
    return (
      <div style={{ padding: '20px', textAlign: 'center', color: 'var(--vscode-descriptionForeground)' }}>
        No nodes match current filters
      </div>
    );
  }

  // Simple layout: arrange nodes in a grid
  const gridSize = Math.ceil(Math.sqrt(filteredNodes.length));
  const nodeWidth = 120;
  const nodeHeight = 40;
  const padding = 20;
  const svgWidth = gridSize * (nodeWidth + padding) + padding;
  const svgHeight = Math.ceil(filteredNodes.length / gridSize) * (nodeHeight + padding) + padding;

  return (
    <div style={{ padding: '10px', overflow: 'auto' }}>
      <svg width={svgWidth} height={svgHeight} style={{ display: 'block' }}>
        {/* Edges */}
        {filteredEdges.map(edge => {
          const sourceNode = nodeMap.get(edge.source);
          const targetNode = nodeMap.get(edge.target);
          if (!sourceNode || !targetNode) return null;

          const sourceIdx = filteredNodes.indexOf(sourceNode);
          const targetIdx = filteredNodes.indexOf(targetNode);
          const sourceX = (sourceIdx % gridSize) * (nodeWidth + padding) + padding + nodeWidth / 2;
          const sourceY = Math.floor(sourceIdx / gridSize) * (nodeHeight + padding) + padding + nodeHeight / 2;
          const targetX = (targetIdx % gridSize) * (nodeWidth + padding) + padding + nodeWidth / 2;
          const targetY = Math.floor(targetIdx / gridSize) * (nodeHeight + padding) + padding + nodeHeight / 2;

          return (
            <line
              key={edge.id}
              x1={sourceX}
              y1={sourceY}
              x2={targetX}
              y2={targetY}
              stroke="#666"
              strokeWidth={1}
              markerEnd="url(#arrowhead)"
            />
          );
        })}

        {/* Arrow marker */}
        <defs>
          <marker
            id="arrowhead"
            markerWidth="10"
            markerHeight="7"
            refX="9"
            refY="3.5"
            orient="auto"
          >
            <polygon points="0 0, 10 3.5, 0 7" fill="#666" />
          </marker>
        </defs>

        {/* Nodes */}
        {filteredNodes.map((node, idx) => {
          const x = (idx % gridSize) * (nodeWidth + padding) + padding;
          const y = Math.floor(idx / gridSize) * (nodeHeight + padding) + padding;

          return (
            <g key={node.id}>
              <rect
                x={x}
                y={y}
                width={nodeWidth}
                height={nodeHeight}
                fill={getNodeColor(node)}
                stroke={getNodeBorder(node)}
                strokeWidth={2}
                rx={4}
                ry={4}
              />
              <text
                x={x + nodeWidth / 2}
                y={y + nodeHeight / 2}
                textAnchor="middle"
                dominantBaseline="middle"
                fill="white"
                fontSize={10}
              >
                {node.label.length > 15 ? node.label.slice(0, 12) + '...' : node.label}
              </text>
              {node.violationCount > 0 && (
                <circle
                  cx={x + nodeWidth - 8}
                  cy={y + 8}
                  r={8}
                  fill="#f44336"
                />
              )}
              {node.violationCount > 0 && (
                <text
                  x={x + nodeWidth - 8}
                  y={y + 9}
                  textAnchor="middle"
                  dominantBaseline="middle"
                  fill="white"
                  fontSize={8}
                >
                  {node.violationCount}
                </text>
              )}
              <LayerBadge x={x + 4} y={y + 4} layer={node.layer} />
            </g>
          );
        })}
      </svg>
    </div>
  );
}
