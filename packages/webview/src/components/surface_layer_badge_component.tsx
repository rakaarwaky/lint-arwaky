/** Passive Surface: Layer badge component.

Small colored badge showing the AES layer of a node.
*/

import React from 'react';

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

interface LayerBadgeProps {
  x: number;
  y: number;
  layer: string;
}

export function LayerBadge({ x, y, layer }: LayerBadgeProps) {
  const color = LAYER_COLORS[layer] || '#757575';
  const label = layer.charAt(0).toUpperCase();

  return (
    <g>
      <rect
        x={x}
        y={y}
        width={12}
        height={12}
        fill={color}
        rx={2}
        ry={2}
      />
      <text
        x={x + 6}
        y={y + 7}
        textAnchor="middle"
        dominantBaseline="middle"
        fill="white"
        fontSize={8}
        fontWeight="bold"
      >
        {label}
      </text>
    </g>
  );
}
