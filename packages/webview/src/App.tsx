/** Passive Surface: Main React App — receives extension messages, renders UI. */

import React, { useState, useEffect } from 'react';
import { DependencyGraphVO } from './types';
import { useGraphData } from './hooks/useGraphData';
import { DependencyGraph } from './components/DependencyGraph';
import { ViolationList } from './components/ViolationList';

declare function acquireVsCodeApi(): {
  postMessage(msg: unknown): void;
};

export default function App() {
  const [graph, setGraph] = useState<DependencyGraphVO | null>(null);
  const [status, setStatus] = useState<'scanning' | 'complete' | 'error'>('complete');
  const [message, setMessage] = useState('');
  const [filterSeverity, setFilterSeverity] = useState<string | null>(null);
  const [filterLayer, setFilterLayer] = useState<string | null>(null);

  useGraphData({ setGraph, setStatus, setMessage });

  const handleRefresh = () => {
    acquireVsCodeApi().postMessage({ command: 'refreshScan' });
  };

  const handleOpenViolation = (file: string, line: number, column: number) => {
    acquireVsCodeApi().postMessage({ command: 'openViolation', file, line, column });
  };

  const handleFilterSeverity = (severity: string | null) => {
    setFilterSeverity(severity);
  };

  const handleFilterLayer = (layer: string | null) => {
    setFilterLayer(layer);
  };

  if (status === 'scanning') {
    return (
      <div style={{ padding: '20px', textAlign: 'center' }}>
        <div style={{ fontSize: '14px', color: 'var(--vscode-descriptionForeground)' }}>
          Scanning workspace...
        </div>
      </div>
    );
  }

  if (!graph) {
    return (
      <div style={{ padding: '20px', textAlign: 'center' }}>
        <div style={{ fontSize: '14px', color: 'var(--vscode-descriptionForeground)' }}>
          {message || 'No data available'}
        </div>
        <button
          onClick={handleRefresh}
          style={{
            marginTop: '10px',
            padding: '6px 12px',
            background: 'var(--vscode-button-background)',
            color: 'var(--vscode-button-foreground)',
            border: 'none',
            borderRadius: '2px',
            cursor: 'pointer',
          }}
        >
          Refresh
        </button>
      </div>
    );
  }

  return (
    <div style={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      <div style={{ padding: '8px 12px', borderBottom: '1px solid var(--vscode-panel-border)' }}>
        <button
          onClick={handleRefresh}
          style={{
            padding: '4px 8px',
            background: 'var(--vscode-button-background)',
            color: 'var(--vscode-button-foreground)',
            border: 'none',
            borderRadius: '2px',
            cursor: 'pointer',
          }}
        >
          Refresh
        </button>
      </div>
      <div style={{ flex: 1, overflow: 'auto' }}>
        <DependencyGraph
          graph={graph}
          filterSeverity={filterSeverity}
          filterLayer={filterLayer}
        />
      </div>
      <div style={{ borderTop: '1px solid var(--vscode-panel-border)', maxHeight: '300px', overflow: 'auto' }}>
        <ViolationList
          violations={graph.violations}
          filterSeverity={filterSeverity}
          filterLayer={filterLayer}
          onOpenViolation={handleOpenViolation}
          onFilterSeverity={handleFilterSeverity}
          onFilterLayer={handleFilterLayer}
        />
      </div>
    </div>
  );
}
