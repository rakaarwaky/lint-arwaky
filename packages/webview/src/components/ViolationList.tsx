/** Passive Surface: Violation list component.

Displays violations in a table with filtering.
*/

import React, { useMemo } from 'react';
import { ViolationVO } from '../types';

interface ViolationListProps {
  violations: readonly ViolationVO[];
  filterSeverity: string | null;
  filterLayer: string | null;
  onOpenViolation: (file: string, line: number, column: number) => void;
  onFilterSeverity: (severity: string | null) => void;
  onFilterLayer: (layer: string | null) => void;
}

const SEVERITY_BADGES: Record<string, { bg: string; color: string }> = {
  critical: { bg: '#f44336', color: 'white' },
  high: { bg: '#ff5722', color: 'white' },
  medium: { bg: '#ff9800', color: 'white' },
  low: { bg: '#4caf50', color: 'white' },
  info: { bg: '#2196f3', color: 'white' },
};

const SEVERITIES = ['critical', 'high', 'medium', 'low', 'info'];

export function ViolationList({
  violations,
  filterSeverity,
  filterLayer,
  onOpenViolation,
  onFilterSeverity,
}: ViolationListProps) {
  const filtered = useMemo(() => {
    return violations.filter(v => {
      if (filterSeverity && v.severity !== filterSeverity) return false;
      return true;
    });
  }, [violations, filterSeverity]);

  return (
    <div style={{ padding: '8px 12px' }}>
      <div style={{ marginBottom: '8px', display: 'flex', gap: '4px', flexWrap: 'wrap' }}>
        <button
          onClick={() => onFilterSeverity(null)}
          style={{
            padding: '2px 8px',
            background: !filterSeverity ? 'var(--vscode-button-background)' : 'var(--vscode-button-secondaryBackground)',
            color: !filterSeverity ? 'var(--vscode-button-foreground)' : 'var(--vscode-button-secondaryForeground)',
            border: 'none',
            borderRadius: '2px',
            cursor: 'pointer',
            fontSize: '11px',
          }}
        >
          All ({violations.length})
        </button>
        {SEVERITIES.map(sev => {
          const count = violations.filter(v => v.severity === sev).length;
          if (count === 0) return null;
          return (
            <button
              key={sev}
              onClick={() => onFilterSeverity(filterSeverity === sev ? null : sev)}
              style={{
                padding: '2px 8px',
                background: filterSeverity === sev ? SEVERITY_BADGES[sev].bg : 'var(--vscode-button-secondaryBackground)',
                color: filterSeverity === sev ? SEVERITY_BADGES[sev].color : 'var(--vscode-button-secondaryForeground)',
                border: 'none',
                borderRadius: '2px',
                cursor: 'pointer',
                fontSize: '11px',
              }}
            >
              {sev} ({count})
            </button>
          );
        })}
      </div>
      <div style={{ maxHeight: '250px', overflow: 'auto' }}>
        <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: '12px' }}>
          <thead>
            <tr style={{ borderBottom: '1px solid var(--vscode-panel-border)' }}>
              <th style={{ textAlign: 'left', padding: '4px 8px' }}>Code</th>
              <th style={{ textAlign: 'left', padding: '4px 8px' }}>Severity</th>
              <th style={{ textAlign: 'left', padding: '4px 8px' }}>File</th>
              <th style={{ textAlign: 'left', padding: '4px 8px' }}>Line</th>
            </tr>
          </thead>
          <tbody>
            {filtered.map((v, idx) => (
              <tr
                key={`${v.file}-${v.line}-${v.code}-${idx}`}
                style={{ borderBottom: '1px solid var(--vscode-panel-border)', cursor: 'pointer' }}
                onClick={() => onOpenViolation(v.file, v.line, v.column)}
              >
                <td style={{ padding: '4px 8px', fontFamily: 'var(--vscode-editor-font-family)' }}>{v.code}</td>
                <td style={{ padding: '4px 8px' }}>
                  <span style={{
                    padding: '1px 6px',
                    borderRadius: '2px',
                    background: SEVERITY_BADGES[v.severity]?.bg || '#757575',
                    color: SEVERITY_BADGES[v.severity]?.color || 'white',
                    fontSize: '10px',
                  }}>
                    {v.severity}
                  </span>
                </td>
                <td style={{ padding: '4px 8px', maxWidth: '200px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{v.file}</td>
                <td style={{ padding: '4px 8px' }}>{v.line}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
